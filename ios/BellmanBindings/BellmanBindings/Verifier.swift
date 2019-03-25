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

public enum EngineTypes: UInt8 {
    case BLS12_381 = 0
    case Bn256 = 1
}

public final class Verifier {
    public func verifyProof(filename: String, inputs: [UInt8], engine: EngineTypes) throws -> Bool {
        let result = verify(filename, inputs, UInt(inputs.count), engine.rawValue)
        let verificationResult = result.value
        let error = String(cString: result.error!)
        free_memory(result)
        if !error.isEmpty {
            throw VerifyError(description: error)
        }
        return verificationResult
    }
}
