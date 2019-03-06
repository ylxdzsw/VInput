const code = Dict{Char, Any}()
const freq = Dict{Char, Int}()

const tone = Dict(
    'ā' => 'a', 'á' => 'a', 'ǎ' => 'a', 'à' => 'a',
    'ō' => 'o', 'ó' => 'o', 'ǒ' => 'o', 'ò' => 'o',
    'ē' => 'e', 'é' => 'e', 'ě' => 'e', 'è' => 'e',
    'ī' => 'i', 'í' => 'i', 'ǐ' => 'i', 'ì' => 'i',
    'ū' => 'u', 'ú' => 'u', 'ǔ' => 'u', 'ù' => 'u',
    'ü' => 'v', 'ǘ' => 'v', 'ǚ' => 'v', 'ǜ' => 'v'
)

for line in eachline("data/raw/pinyin.raw")
    line[1] == '#' && continue
    m = match(r"^U\+(.*?): (.*?)\s+(#.*)?$", line)
    if m == nothing
        println(stderr, line)
        continue
    end

    char, seq = m[1], m[2]
    seq = filter(x->match(r"^[a-z,]+$", x) != nothing, split(join(replace(x->get(tone, x, x), collect(seq))), ',')) |> unique

    # special code for 嗯, which has the pinyin "ń,ńg,ňg,ň,ǹg,ǹ"
    if char == "55EF"
        seq = ["en"]
    end

    if length(seq) == 0
        println(stderr, line)
        continue
    end

    char = Char(parse(Int32, "0x$char"))
    code[char] = seq
    freq[char] = 0
end

# may kill it before actually finished
const corpus = ("data/raw/corpus_1", "data/raw/corpus_2", "data/raw/corpus_3", "data/raw/corpus_4")
for file in reverse(corpus), line in eachline(file)
    i = findfirst(x->x==' ', line)
    w = parse(Int, line[1:i-1])
    for c in line[i+1:end]
        if c in keys(freq)
            freq[c] += w
        end
    end
end

const chars = map(car, sort(collect(freq), by=cadr, rev=true))[1:8192]
const codings = Dict{String, Vector{Char}}()

for char in chars, token in code[char]
    if token in keys(codings)
        push!(codings[token], char)
    else
        codings[token] = [char]
    end
end

open("pinyin.txt", "w") do f
    for (key, value) in sort(collect(codings))
        println(f, key, ' ', join(value))
    end
end