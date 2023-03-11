//
//  ContentView.swift
//  iOS
//
//  Created by complexityclass on 11/03/2023.
//

import Serde
import SwiftUI

enum Message {
    case message(Event)
}

@MainActor
class Model: ObservableObject {
    @Published var view = ViewModel(count: "")
    
    init() {
        update(msg: .message(.reset))
    }
    
    func update(msg: Message) {
        let requests: [Request]
        switch msg {
        case let .message(m):
            requests = try! [Request].bcsDeserialize(input: iOS.processEvent(try! m.bcsSerialize()))
        }
        
        for req in requests {
            switch req.effect {
            case .render(_):
                view = try! ViewModel.bcsDeserialize(input: iOS.view())
            }
        }
        
    }
}

struct ActionButton: View {
    var label: String
    var color: Color
    var action: () -> Void

    init(label: String, color: Color, action: @escaping () -> Void) {
        self.label = label
        self.color = color
        self.action = action
    }

    var body: some View {
        Button(action: action) {
            Text(label)
                .fontWeight(.bold)
                .font(.body)
                .padding(EdgeInsets(top: 10, leading: 15, bottom: 10, trailing: 15))
                .background(color)
                .cornerRadius(10)
                .foregroundColor(.white)
                .padding()
        }
    }
}

struct ContentView: View {
    @ObservedObject var model: Model
    
    var body: some View {
        VStack {
            Image(systemName: "globe")
                .imageScale(.large)
                .foregroundColor(.accentColor)
            Text(model.view.count)
            HStack {
                ActionButton(label: "Inc", color: .green) {
                    model.update(msg: .message(.increment))
                }
                ActionButton(label: "Dec", color: .red) {
                    model.update(msg: .message(.decrement))
                }
            }
        }
        .padding()
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView(model: Model())
    }
}
