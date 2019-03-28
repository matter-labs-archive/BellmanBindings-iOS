//
//  BellmanBindingsTests.swift
//  BellmanBindingsTests
//
//  Created by Anton on 28/03/2019.
//  Copyright © 2019 TheMatter. All rights reserved.
//

import XCTest
@testable import BellmanBindings_iOS

class BellmanBindingsTests: XCTestCase {
    
    func testVerification() {
        let verifier = Verifier()
        let downloader = Downloader()
        do {
            let inputsString: String = "0x0ac9e6cd1b0607c7d618fc66501a362d7190ca18e9022f7e5a3b305495ecd246"
            let inputsBytes: [UInt8] = inputsString.hexa2Bytes
            let proofVec: [UInt8] = [28, 95, 108, 21, 41, 144, 204, 16, 73, 127, 208, 194, 228, 103, 7, 64, 51, 72, 144, 104, 228, 230, 26, 38, 50, 46, 241, 61, 136, 200, 2, 189, 38, 11, 158, 231, 202, 192, 148, 210, 65, 88, 106, 77, 5, 251, 76, 183, 212, 103, 26, 161, 251, 255, 11, 113, 91, 194, 105, 137, 99, 155, 96, 138, 46, 104, 191, 196, 85, 63, 95, 73, 150, 99, 201, 69, 130, 79, 22, 151, 237, 150, 50, 173, 223, 173, 146, 151, 131, 239, 233, 255, 38, 61, 93, 21, 168, 14, 179, 37, 112, 193, 17, 28, 247, 226, 193, 213, 103, 235, 136, 193, 171, 198, 192, 166, 203, 19, 79, 26, 243, 30, 173, 3, 246, 60, 83, 78]
            
            let result = try verifier.verifyWithPrecompiledProof(filename: "vk.txt",
                                                                 inputs: inputsBytes,
                                                                 engine: Engine.Bn256,
                                                                 proofVec: proofVec)
            print(result)
            XCTAssert(result)
        } catch let error {
            print((error as! VerifyError).description)
            XCTFail()
        }
    }

}


