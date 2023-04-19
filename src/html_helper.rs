use crate::parser::lang::member::Member;
use crate::parser::lang::method::Method;

pub struct HtmlHelper {}

impl HtmlHelper {
    pub fn str_header() -> String {
r#"<html>

<header>
<link rel="stylesheet" href="euphoria.css">
</header>

<body>
<div class="container">
"#.to_string()
    }

    pub fn str_footer() -> String {
r#"
</div>
</body>

<footer>
</footer>

</html>"#.to_string()
    }

    pub fn str_member_list() -> String {
r#"
<h2 class="heading2">Members</h2>
"#.to_string()
    }

    pub fn gen_member(member: &Member) -> (String, String) {
        let mut id = String::new();
        if let Some(declare) = &member.declare {
            for c in declare.chars() {
                if c.is_alphanumeric() {
                    id.push(c);
                } else if c == '_' {
                    id.push('-');
                } else if c == ' ' || c == '<' || c == '>' {
                    id.push('_');
                }
            }
        } else {
            id = uuid::Uuid::new_v4().to_string();
        }

        let mut member_str = String::new();
        member_str += format!("<div class=\"member-item-container\" id=\"{}\">", id).as_str();
        if let Some(declare) = &member.declare {
            member_str += format!("<pre class=\"member-declare-container\"><code>{}</code></pre>\n", HtmlHelper::preprocess_source(declare.as_str())).as_str();
        }
        member_str += format!("<p class=\"member-uproperty\">It is{} a UPROPERTY.</p>\n", if member.has_uproperty { "" } else { " not" }).as_str();
        if let Some(desc) = &member.desc {
            member_str += r#"<div class="member-desc-container">"#;
            member_str += r#"<h3 class="heading3">Desc</h3>"#;

            let desc_splited_by_newline = &desc.description.clone();
            let desc_splited_by_newline = desc_splited_by_newline.split("\n");
            for line in desc_splited_by_newline {
                member_str += format!("<p class=\"member-desc\">{}</p>", line).as_str();
            }
            member_str += r#"</div>"#;
        }
        member_str += r#"</div>"#;
        member_str += "\n\n";

        (member_str, id)
    }

    pub fn str_method_list() -> String {
        r#"
<h2 class="heading2">Methods</h2>
"#.to_string()
    }

    pub fn gen_method(method: &Method) -> String {
        format!("<p>method: {:?}</p>", method)
    }

    pub fn gen_url(url: &str, content: &str) -> String {
        format!("<p><a href=\"{}\">{}</a></p>", url, content)
    }

    pub fn gen_heading(size: usize, content: &str) -> String {
        assert!(1 <= size && size <= 6);
        format!("<h{0} class=\"heading{0}\">{1}</h{0}>", size, content)
    }

    fn preprocess_source(source: &str) -> String {
        let mut ret = String::new();
        for c in source.chars() {
            if c == '<' || c == '>' {
                ret += format!("<span>{}</span>", c).as_str();
            } else {
                ret.push(c);
            }
        }
        ret
    }
}
