//
//  ViewController.swift
//  BellmanBindings
//
//  Created by Anton on 25/03/2019.
//  Copyright © 2019 TheMatter. All rights reserved.
//

import UIKit

class ViewController: UIViewController {

    override func viewDidLoad() {
        super.viewDidLoad()
        let verifier = Verifier()
        do {
            let result = try verifier.verifyProof(filename: "dfsdf", inputs: [0,0,1], engine: Engine.Bn256)
            print(result)
        } catch let error {
            print(error.localizedDescription)
        }
    }
}

