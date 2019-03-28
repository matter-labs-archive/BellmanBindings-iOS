//
//  Downloader.swift
//  BellmanBindings-iOS
//
//  Created by Anton on 28/03/2019.
//  Copyright © 2019 TheMatter. All rights reserved.
//

import Foundation

class Downloader {
    func loadFileAsync(url: URL) throws -> String {
        var filepath: String? = nil
        var loadError: Error? = nil
        
        let group = DispatchGroup()
        group.enter()

        let documentsUrl =  FileManager.default.urls(for: .documentDirectory, in: .userDomainMask).first!
        
        let destinationUrl = documentsUrl.appendingPathComponent(url.lastPathComponent)
        
        if FileManager().fileExists(atPath: destinationUrl.path) {
            filepath = destinationUrl.path
            group.leave()
        } else {
            let session = URLSession(configuration: URLSessionConfiguration.default, delegate: nil, delegateQueue: nil)
            var request = URLRequest(url: url)
            request.httpMethod = "GET"
            let task = session.downloadTask(with: request) { (path, response, error) in
                if error == nil {
                    if let response = response as? HTTPURLResponse {
                        if response.statusCode == 200 {
                            if let path = path {
                                filepath = path.path
                                group.leave()
                            } else {
                                loadError = CustomError(description: "Can't get filepath")
                                group.leave()
                            }
                        } else {
                            loadError = CustomError(description: "Wrong response code")
                            group.leave()
                        }
                    } else {
                        loadError = CustomError(description: "Wrong response")
                        group.leave()
                    }
                } else {
                    loadError = CustomError(description: error?.localizedDescription ?? "Download error")
                    group.leave()
                }
            }
            task.resume()
        }
        group.wait()
        guard loadError == nil, filepath != nil else{
            throw loadError!
        }
        return filepath!
    }
}
