//
//  ContentView.swift
//  iOS
//
//  Created by complexityclass on 11/03/2023.
//

import Serde
import SwiftUI

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
    @State private var input = ""
    
    var body: some View {
        VStack {
            Text(model.view.spent).foregroundColor(.cyan)
            AsyncImage(url: URL(string: model.view.img_url)!) { image in
                image
                    .resizable(resizingMode: .stretch)
                    .aspectRatio(contentMode: .fit)
                    .frame(width: 300, height: 300)
            } placeholder: {
                Color.green
            }
            TextField("Add some context...", text: $input)
                .padding()
                .font(.system(size: 20))
                .cornerRadius(10)
                .padding()
            Text(model.view.answer)
            HStack {
                ActionButton(label: "Write!", color: .green) {
                    guard input.count > 10 else { return }
                    model.update(msg: .message(.ask(input)))
                }
                Text("🤖")
                ActionButton(label: "Draw!", color: .purple) {
                    guard input.count > 10 else { return }
                    model.update(msg: .message(.gen(input)))
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
