using Fire

const bots = ("bitmessage", "bitgo", "ShowBotCommand", "CNBlackListR", "请先阅读群置顶消息", "最新CNY价")

@main function main(files...)
    for file in files, m in eachmatch(r"<div class=\"text\">(.*?)</div>"s, read(file, String)), line in split(strip(m[1]), '\n')
        any(keyword -> occursin(keyword, line), bots) && continue
        println("5 ", line)
    end
end