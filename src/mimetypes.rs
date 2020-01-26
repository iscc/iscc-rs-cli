static enum GMT = {
    TEXT,
    IMAGE,
    AUDIO,
    VIDEO,
};

static struct SUPPORTED_MIME_TYPES = {
    // Text Formats
    "application/rtf": {"gmt": GMT::TEXT, "ext": "rtf"},
    "application/msword": {"gmt": GMT::TEXT, "ext": "doc"},
    "application/pdf": {"gmt": GMT::TEXT, "ext": "pdf"},
    "application/epub+zip": {"gmt": GMT::TEXT, "ext": "epub"},
    "application/xml": {"gmt": GMT::TEXT, "ext": "xml"},
    "application/vnd.oasis.opendocument.text": {"gmt": GMT::TEXT, "ext": "odt"},
    "text/html": {"gmt": GMT::TEXT, "ext": "html"},
    "text/plain": {"gmt": GMT::TEXT, "ext": "txt"},
    "application/x-ibooks+zip": {"gmt": GMT::TEXT, "ext": "ibooks"},
    "text/x-web-markdown": {"gmt": GMT::TEXT, "ext": "md"},
    "application/vnd.openxmlformats-officedocument.wordprocessingml.document": {
        "gmt": GMT::TEXT,
        "ext": "docx",
    },
    // Image Formats
    "image/gif": {"gmt": GMT::IMAGE, "ext": "gif"},
    "image/jpeg": {"gmt": GMT::IMAGE, "ext": ["jpg", "jpeg"]},
    "image/png": {"gmt": GMT::IMAGE, "ext": "png"},
    "image/tiff": {"gmt": GMT::IMAGE, "ext": "tif"},
    "application/postscript": {"gmt": GMT::IMAGE, "ext": "eps"},
    // Audio Formats
    "audio/mpeg": {"gmt": GMT::AUDIO, "ext": "mp3"},
    "audio/vnd.wave": {"gmt": GMT::AUDIO, "ext": "wav"},
    "audio/vorbis": {"gmt": GMT::AUDIO, "ext": "ogg"},
    "audio/x-aiff": {"gmt": GMT::AUDIO, "ext": "aif"},
    // Video Formats
    "application/vnd.rn-realmedia": {"gmt": GMT::VIDEO, "ext": "rm"},
    "video/x-dirac": {"gmt": GMT::VIDEO, "ext": "drc"},
    "video/3gpp": {"gmt": GMT::VIDEO, "ext": "3gp"},
    "video/3gpp2": {"gmt": GMT::VIDEO, "ext": "3g2"},
    "video/x-ms-asf": {"gmt": GMT::VIDEO, "ext": "asf"},
    "video/x-msvideo": {"gmt": GMT::VIDEO, "ext": "avi"},
    "video/webm": {"gmt": GMT::VIDEO, "ext": "webm"},
    "video/mpeg": {"gmt": GMT::VIDEO, "ext": ["mpeg", "mpg", "m1v", "vob"]},
    "video/mp4": {"gmt": GMT::VIDEO, "ext": "mp4"},
    "video/x-m4v": {"gmt": GMT::VIDEO, "ext": "m4v"},
    "video/x-matroska": {"gmt": GMT::VIDEO, "ext": "mkv"},
    "video/theora": {"gmt": GMT::VIDEO, "ext": ["ogg", "ogv"]},
    "video/quicktime": {"gmt": GMT::VIDEO, "ext": ["mov", "f4v"]},
    "video/x-flv": {"gmt": GMT::VIDEO, "ext": "flv"},
    "application/x-shockwave-flash": {"gmt": GMT::VIDEO, "ext": "swf"},
    "video/h264": {"gmt": GMT::VIDEO, "ext": "h264"},
    "video/x-ms-wmv": {"gmt": GMT::VIDEO, "ext": "wmv"},
};
