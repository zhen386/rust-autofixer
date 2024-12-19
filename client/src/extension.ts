import * as path from 'path';
import { workspace, ExtensionContext, commands, window } from 'vscode';
import { Executable, LanguageClient, LanguageClientOptions, ServerOptions } from 'vscode-languageclient/node';
import { spawn } from 'child_process';

let client: LanguageClient;

export async function activate(context: ExtensionContext) {
    function readMessage(): Promise<string> {
        return new Promise((resolve, reject) => {
          let dataBuffer = '';
          let contentLength = 0;
    
          const onData = (chunk: Buffer) => {
            dataBuffer += chunk.toString('utf8');
    
            while (true) {
              if (contentLength === 0) {
                const headerEndIndex = dataBuffer.indexOf('\r\n\r\n');
                if (headerEndIndex === -1) {
                  // Not enough data for headers
                  break;
                }
    
                const header = dataBuffer.slice(0, headerEndIndex);
                const lines = header.split('\r\n');
                for (const line of lines) {
                  const [name, value] = line.split(': ');
                  if (name.toLowerCase() === 'content-length') {
                    contentLength = parseInt(value, 10);
                    break;
                  }
                }
    
                if (contentLength === 0) {
                  reject(new Error('Invalid Content-Length header'));
                  return;
                }
    
                dataBuffer = dataBuffer.slice(headerEndIndex + 4);
              }
    
              if (dataBuffer.length >= contentLength) {
                const message = dataBuffer.slice(0, contentLength);
                dataBuffer = dataBuffer.slice(contentLength);
                contentLength = 0;
    
                stdout.off('data', onData);
                resolve(message);
                console.log('Received message:', message);
                break;
              } else {
                // Not enough data for the message body
                break;
              }
            }
          };
    
          stdout.on('data', onData);
          stdout.on('error', reject);
        });
      }
    console.log('Activating');
    const exePath = context.asAbsolutePath(path.join('server', 'vfs-demo.exe'));
    const run: Executable = {
        command: exePath,
    };

    const serverOptions: ServerOptions = {
        run: run,
        debug: run
    };

    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'rust' }],
        synchronize: {
            fileEvents: workspace.createFileSystemWatcher('**/*.rs')
        }
    };

    client = new LanguageClient(
        'languageServerExample',
        'Language Server Example',
        serverOptions,
        clientOptions
    );
    // 启动LanguageClient，这会自动启动LSP服务器
    client.start();
    console.log('Activated');

    const stdout = spawn(exePath, [], {
        stdio: ['pipe', 'pipe', 'inherit'],
      }).stdout;

    // Read response
    const response = await readMessage();
    console.log('Received:', response);


}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

