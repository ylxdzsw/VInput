import * as vscode from 'vscode'
import { TextEditor, TextEditorEdit } from 'vscode';
import * as dgram from 'dgram';

const vInput = {
    server: null,
    editor: null,

    listen() {
        this.editor = vscode.window.activeTextEditor || this.editor
        if (this.server) return
        this.server = dgram.createSocket('udp4')
        this.server.bind(22335)
        this.server.on('message', (msg, rinfo) => {
            if (msg[0] == 0) { // control
                switch (msg[1]) {
                    case 1: vscode.commands.executeCommand('cursorUp'); break
                    case 2: vscode.commands.executeCommand('cursorRight'); break
                    case 3: vscode.commands.executeCommand('cursorDown'); break
                    case 4: vscode.commands.executeCommand('cursorLeft'); break
                    case 5: vscode.commands.executeCommand('deleteLeft'); break
                    case 6: vscode.commands.executeCommand('deleteRight'); break
                    case 7: vscode.commands.executeCommand('cursorHome'); break
                    case 8: vscode.commands.executeCommand('cursorEnd'); break
                }
            } else {
                this.insert(''+msg)
            }
        })
    },

    insert(value) {
        this.editor.edit(builder => this.editor.selection.isEmpty
            ? builder.insert(this.editor.selection.active, value)
            : builder.replace(this.editor.selection, value))
    }
}

export function activate(context: vscode.ExtensionContext) {
    context.subscriptions.push(vscode.commands.registerCommand('vinput.activate', () => {
        vInput.listen()
    }))
}

export function deactivate() {
    
}
