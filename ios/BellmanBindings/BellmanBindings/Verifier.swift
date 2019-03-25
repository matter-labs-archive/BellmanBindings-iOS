//
//  Verifier.swift
//  BellmanBindings
//
//  Created by Anton on 25/03/2019.
//  Copyright © 2019 TheMatter. All rights reserved.
//

import Foundation

public class VerifyError: Error {
    var localizedDescription: String
    init(description: String) {
        localizedDescription = description
    }
}

class Verifier {
    func verifyProof(filename: String, inputs: [UInt8]) throws -> Bool {
        let result = verify(filename, inputs, UInt(inputs.count))
        let verificationResult = result.value
        let error = String(cString: result.error!)
        free_memory(result)
        if !error.isEmpty {
            throw VerifyError(description: error)
        }
        return verificationResult
    }
}
