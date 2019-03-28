//
//  StringProtocolExtension.swift
//  BellmanBindings
//
//  Created by Anton on 28/03/2019.
//  Copyright © 2019 TheMatter. All rights reserved.
//

import Foundation

extension StringProtocol {
    var hexa2Bytes: [UInt8] {
        var start = startIndex
        return stride(from: 0, to: count, by: 2).compactMap {  _ in
            let end = index(start, offsetBy: 2, limitedBy: endIndex) ?? endIndex
            defer { start = end }
            return UInt8(self[start..<end], radix: 16)
        }
    }
}
