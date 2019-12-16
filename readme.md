VInput
======

A experimental new Chinese input method. (not done yet)

#### open source & corss platform

VInput is supposed to train on your own data (see below), so there is no need for us to collect any stat. You privacy never leaks.

#### simple and fast

Latency is an important factor for an IM. Thanks to Rust, VInput provides a consistant acceptable latency while using relatively large models.

#### train the model on your own

Full sentence input requires trainning a language model, which typically requires a large corpus. However, VInput use innovative techniques to allow you to train the model on a very small dataset. Simply export your emails/blogs/chat histories and train a model that knows you best.

VInput use less corpus by:

- **Corpus Enhancing**: VInput use word embedding (if you don't know what's that, think of AI or deep learning on the market) to enhance the trainning data, by replcaing some words with other words that has similar usage)
- **Bayesian Inference**: VInput treat your data differently with People's Daily / Wikipedia / Sougo news / whatever corpus not yours. The "base" model trained with public data is treat as *priori* and your data is *evidence*, and VInput will calculate the posteriori probability for use.
