//
//  ContentView.swift
//  App
//
//  Created by Remco Bloemen on 1/2/23.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        Button(action: {
            run()
        }, label: {
            Text("Run!")
        })
    }
}

struct ContentView_Previews: PreviewProvider {
    static var previews: some View {
        ContentView()
    }
}
