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
            insert(this.editor, ''+msg)
        })
    }
}

export function activate(context: vscode.ExtensionContext) {
    context.subscriptions.push(vscode.commands.registerCommand('vinput.activate', () => {
        vInput.listen()
    }))
}

export function deactivate() {
    
}
