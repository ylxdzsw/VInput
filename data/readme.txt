char_id: every line is a character, sorted by unigram frequency.
freq: each line is the logarithm of correspoding unigram frequency.
skip4: a row-major binary array, each position is a 4-tuple of 32bit float number representing `log(p(w_k|w_(k-N)))`
[encodings]: each line contains several elements separated by space. the first element is a sequence of a-z and remaining elements are the id of the characters that is encoded by the sequence.