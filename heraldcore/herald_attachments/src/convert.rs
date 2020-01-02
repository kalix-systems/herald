use super::*;

impl From<Vec<String>> for AttachmentMeta {
    fn from(v: Vec<String>) -> AttachmentMeta {
        Self(v)
    }
}

impl From<DocMeta> for json::JsonValue {
    fn from(meta: DocMeta) -> json::JsonValue {
        let DocMeta { path, name, size } = meta;

        use json::object;

        object! {
            "path" => path,
            "name" => name,
            "size" => size
        }
    }
}

impl From<MediaMeta> for json::JsonValue {
    fn from(meta: MediaMeta) -> json::JsonValue {
        use json::object;

        let MediaMeta {
            path,
            width,
            height,
            name,
        } = meta;

        object! {
            "path" => path,
            "width" => width,
            "height" => height,
            "name" => name,
        }
    }
}

impl From<Docs> for json::JsonValue {
    fn from(docs: Docs) -> json::JsonValue {
        use json::object;
        let Docs { items, num_more } = docs;

        object! {
            "items" => items,
            "num_more" => num_more,
        }
    }
}

impl From<Media> for json::JsonValue {
    fn from(media: Media) -> json::JsonValue {
        use json::object;
        let Media { items, num_more } = media;

        object! {
            "items" => items,
            "num_more" => num_more,
        }
    }
}
