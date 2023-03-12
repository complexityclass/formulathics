//
//  Model.swift
//  iOS
//
//  Created by complexityclass on 11/03/2023.
//

import Foundation
import Serde
import SwiftUI

typealias Uuid = [UInt8]

enum Outcome {
    case http(HttpResponse)
}

enum Message {
    case message(Event)
    case response(Uuid, Outcome)
}

@MainActor
class Model: ObservableObject {
    @Published var view = ViewModel(answer: "", img_url: "https://example.com/icon.png", spent: "Nothing spent yet!")
        
    init() {
        update(msg: .message(.reset))
    }
    
    func update(msg: Message) {
        let requests: [Request]
        switch msg {
        case let .message(m):
            requests = try! [Request].bcsDeserialize(input: iOS.processEvent(try! m.bcsSerialize()))
        case let .response(uuid, outcome):
            requests = try! [Request].bcsDeserialize(input: iOS.handleResponse(uuid, {
                switch outcome {
                case let .http(v):
                    return try! v.bcsSerialize()
                }
            }()))
        }
        
        for req in requests {
            switch req.effect {
            case .render(_):
                view = try! ViewModel.bcsDeserialize(input: iOS.view())
            case .http(let hr):
                http(uuid: req.uuid, method: hr.method, url: hr.url.removingPercentEncoding!)
            }
        }
    }
    
    private func http(uuid: Uuid, method: String, url: String) {
        let components = url.split(separator: "£").map { String($0) }
        assert(components.count == 3)
        var request = URLRequest(url: URL(string: components[0])!)
        request.httpMethod = method
        request.addValue(components[1], forHTTPHeaderField: "Authorization")
        request.addValue("application/json", forHTTPHeaderField: "Content-Type")
        request.httpBody = components[2].data(using: .utf8)
        
        Task {
            let (data, response) = try! await URLSession.shared.data(for: request)
            if let httpResponse = response as? HTTPURLResponse {
                let status = UInt16(httpResponse.statusCode)
                let str = String(data: data, encoding: .utf8)
                let body = [UInt8](data)
                self.update(msg: .response(uuid, .http(HttpResponse(status: status, body: body))))
            }
        }
    }
}
