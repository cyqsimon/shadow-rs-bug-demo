fn main() {
    shadow_rs::shadow!(build);
    println!(
        "value: `{}`; type: `{}`",
        build::COMMITS_SINCE_TAG,
        std::any::type_name_of_val(&build::COMMITS_SINCE_TAG),
    );
}
