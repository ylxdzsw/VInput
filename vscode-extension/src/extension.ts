import * as vscode from 'vscode'
import { TextEditor, TextEditorEdit } from 'vscode';
import * as dgram from 'dgram';

const insert = (editor: TextEditor, value: string) =>
    editor.edit(builder => editor.selection.isEmpty
        ? builder.insert(editor.selection.active, value)
        : builder.replace(editor.selection, value))

const vInput = {
    server: null,
    editor: null,

    listen() {
        this.editor = vscode.window.activeTextEditor || this.editor
        this.server = dgram.createSocket('udp4')
        this.server.bind(22335)
        this.server.on('message', (msg, rinfo) => {
            if (msg[0] == 0) { // control
                switch (msg[1]) {
                    case 1: this.move(-1, 0); break
                    case 2: this.move(0, 1); break
                    case 3: this.move(1, 0); break
                    case 4: this.move(0, -1); break
                }
            } else {
                insert(this.editor, ''+msg)
            }
        })
    },

    move(x, y) {
        const pos = this.editor.selection.active
        const newpos = pos.with(pos.line + x, pos.character + y)
        this.editor.selection = new vscode.Selection(newpos, newpos)
    }
}

export function activate(context: vscode.ExtensionContext) {
    context.subscriptions.push(vscode.commands.registerCommand('vinput.activate', () => {
        vInput.listen()
    }))
}

export function deactivate() {
    
}
