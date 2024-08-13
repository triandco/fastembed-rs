use ndarray::{s, ArrayBase, Dim, IxDynImpl, OwnedRepr, ViewRepr};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Pooling {
    Cls,
    Mean,
}

/// Pool the previous layer output by simply taking the CLS (presumably the first) token embedding and using this as the sentence embedding
/// * `token_embeddings` - token embeddings in form of a tensor output of the encoding.
// Please refer to the original python implementation for more details:
// https://github.com/UKPLab/sentence-transformers/blob/c0fc0e8238f7f48a1e92dc90f6f96c86f69f1e02/sentence_transformers/models/Pooling.py#L141
pub fn cls(
    token_embeddings: &ArrayBase<ViewRepr<&f32>, Dim<IxDynImpl>>,
) -> ArrayBase<OwnedRepr<f32>, Dim<[usize; 2]>> {
    token_embeddings.slice(s![.., 0, ..]).to_owned()
}

/// Pool the previous layer output by taking the element-wise arithmetic mean of the token-level embeddings after applying the attention mask.
/// * `token_embeddings` - token embeddings in form of a tensor output of the encoding.
/// * `attention_mask` - is the same mask generated by Tokenizer and used for encoding.
// Please refer to the original python implementation for more details:
// https://github.com/UKPLab/sentence-transformers/blob/c0fc0e8238f7f48a1e92dc90f6f96c86f69f1e02/sentence_transformers/models/Pooling.py#L151
pub fn mean(
    token_embeddings: &ArrayBase<ViewRepr<&f32>, Dim<IxDynImpl>>,
    attention_mask: ArrayBase<OwnedRepr<i64>, Dim<[usize; 2]>>,
) -> ArrayBase<OwnedRepr<f32>, Dim<IxDynImpl>> {
    let input_mask_expanded = attention_mask
        .insert_axis(ndarray::Axis(2))
        .broadcast(token_embeddings.dim())
        .expect("Resize attention mask to match output successfull")
        .mapv(|x| x as f32);

    let sum_embeddings = (token_embeddings * &input_mask_expanded).sum_axis(ndarray::Axis(1));
    let sum_mask = input_mask_expanded.sum_axis(ndarray::Axis(1));
    let sum_mask = sum_mask.mapv(|x| if x == 0f32 { 1.0 } else { x }); // clamp to avoid 0 division
    &sum_embeddings / &sum_mask
}
