#!/usr/bin/env julia

# 1. parse encoding, get a list of input-able characters, giving them a temperary id
# 2. iterate the corpus, records a big table of skip N gram and unigram
# 3. sort the list of characters with unigram and re-assign id
# 4. cut the tables with only frequent characters and map to new id
# 5. generate binary dict file

using OhMyJulia

const Nskip = 4
const freq_threshold = 4095

function parse_encoding(file)
    Set{Char}(c for line in eachline(file) for c in cadr(split(line, ' '))) |> collect
end

function emurate_corpus(id_map, files...)
    N = length(id_map)
    unigram, skips = fill(0, N+1), ntuple(x->fill(0, N+1, N+1), Nskip)
    for file in files, line in eachline(file)
        i = findfirst(x->x==' ', line)
        w = parse(Int, line[1:i-1])
        r = collect(line[i+1:end])

        # first pass: collect unigram and split
        s = [0]
        for (j, c) in enumerate(r)
            id = get(id_map, c, 0)
            unigram[id+1] += w
            id == 0 && push!(s, j)
        end
        push!(s, length(r)+1)
        
        # second pass: collect N gram
        for j in 1:length(s)-1
            seg = map(x->id_map[x]+1, r[s[j]+1:s[j+1]-1])
            for k in 1:length(seg), n in 1:Nskip
                if k <= n
                    skips[n][1, seg[k]] += w
                else
                    skips[n][seg[k-n], seg[k]] += w
                end
            end
        end
    end

    unigram, skips
end

encoding = "data/raw/pinyin.txt"
corpus = ("data/raw/corpus_1", "data/raw/corpus_2", "data/raw/corpus_3")

t_id = parse_encoding(encoding)
t_map = Dict(c=>i for (i, c) in enumerate(t_id))
t_unigram, t_skips = emurate_corpus(t_map, corpus...)
perm = sortperm(t_unigram[2:end], rev=true)
unigram = t_unigram[perm .+ 1]
t = 1 ++ (perm[1:freq_threshold] .+ 1)
skips = map(x->x[t, t], t_skips)
open("data/char_id", "w") do f
    print(f, join(t_id[perm], '\n'))
end
open("data/pinyin", "w") do f
    for line in eachline(encoding)
        enc, cont = split(line, ' ')
        println(f, replace(enc, 'ü' => 'v'), ' ', join(map(c->findfirst(x->x==c, t_id[perm]), collect(cont)), ' '))
    end
end
open("data/freq","w") do f
    u = unigram .+ 1 # laplace smoothing
    write(f, 0f0, f32.(log.(u ./ sum(u))))
end
open("data/skip4", "w") do f
    # explicitly enforce row-major layout
    for i in 1:length(1:1+freq_threshold)
        # laplace smoothing
        lines = map(x->x[i, :] .+ 1, skips)
        lines = map(x->f32.(log.(x ./ sum(x))), lines)
        for j in 1:length(1:1+freq_threshold)
            for k in 1:Nskip
                write(f, f32(lines[k][j]))
            end
        end
    end
end