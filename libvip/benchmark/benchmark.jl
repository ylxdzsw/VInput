using UnicodePlots
using Statistics

const cases = let list = readlines("data/raw/test.txt")
    [(replace(list[i], ' ' => "") |> lowercase, list[i+1]) for i in 1:2:length(list)]
end

const libvip = "target/release/libvip.so"

const ctx = ccall((:init, libvip), Ptr{Nothing}, (Cstring, ), abspath("data"))

correct = 0
times = Float64[]

for (k, v) in cases
    r, t = @timed begin
        ccall((:set_input, libvip), Nothing, (Ptr{Nothing}, Cstring), ctx, k)
        ccall((:get_candidates, libvip), Cstring, (Ptr{Nothing}, ), ctx)
    end
    sentence = match(r" (.*?)\n", unsafe_string(r))[1]
    ccall((:free_candidates, libvip), Nothing, (Cstring, ), r)
    push!(times, t)
    if sentence == v
        global correct += 1
    else
        # println(k)
        # println(v)
        # println(sentence)
        # println()
    end
end

println("\n\n\n ===== result =====")
println("accuracy: $correct / $(length(cases)) = $(100correct / length(cases))%")
println("time: avg=$(1000mean(times))ms, max=$(1000maximum(times))ms")
histogram(1000 .* times)