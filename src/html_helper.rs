use crate::parser::lang::member::Member;
use crate::parser::lang::method::Method;
use std::fmt::format;
use std::ops::Deref;

pub struct HtmlHelper {}

impl HtmlHelper {
    pub fn str_header() -> String {
        r#"<html>

<header>
<link rel="stylesheet" href="euphoria.css">
</header>

<body>
<div class="container">
"#
        .to_string()
    }

    pub fn str_footer() -> String {
        r#"
</div>
</body>

<footer>
</footer>

</html>"#
            .to_string()
    }

    pub fn str_member_list(toc: &mut String) -> String {
        *toc += r#"<h3 class="heading3">Members</h3>"#;
        r#"
<h2 class="heading2">Members</h2>
"#
        .to_string()
    }

    pub fn gen_member(member: &Member, toc: &mut String) -> (String, String) {
        let mut id = String::new();
        id += "v_";
        let mut name_for_toc = String::new();
        if let Some(declare) = &member.declare {
            name_for_toc = declare.clone();
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
            name_for_toc = id.clone();
        }

        let mut member_str = String::new();
        member_str += format!("<div class=\"member-item-container\" id=\"{}\">\n", id).as_str();
        if let Some(name) = &member.name {
            member_str += format!("<h3 class=\"heading3\">{}</h3>", name).as_str();
            *toc += HtmlHelper::gen_url(format!("#{}", id).as_str(), name.as_str()).as_str();
        } else {
            member_str += r#"<h3 class="heading3">MISSING_NAME</h3>"#;
            *toc += HtmlHelper::gen_url(format!("#{}", id).as_str(), id.as_str()).as_str();
        }
        member_str += "\n";

        if let Some(declare) = &member.declare {
            let mut source = String::new();
            if let Some(uproperty) = &member.uproperty {
                source += uproperty.as_str();
                source.push('\n');
            }
            source += HtmlHelper::preprocess_source(declare).as_str();

            member_str += format!(
                "<pre class=\"member-declare-container\">\n<code>{}</code>\n</pre>\n",
                source,
            )
            .as_str();
        }

        if let Some(desc) = &member.desc {
            member_str += r#"<div class="member-desc-container">"#;
            member_str += "\n";

            member_str +=
                HtmlHelper::preprocess_desc("member-desc", desc.description.as_str()).as_str();
            member_str += r#"</div>"#;
            member_str += "\n";
        }
        member_str += r#"</div>"#;
        member_str += "\n\n";

        (member_str, id)
    }

    pub fn str_method_list(toc: &mut String) -> String {
        *toc += r#"<h3 class="heading3">Methods</h3>"#;
        r#"
<h2 class="heading2">Methods</h2>
"#
        .to_string()
    }

    pub fn gen_method(method: &Method, toc: &mut String) -> (String, String) {
        let mut id = String::new();
        id += "f_";
        if let Some(signature) = &method.signature {
            for c in signature.chars() {
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

        let mut method_str = String::new();
        method_str += format!("<div id=\"{}\" class=\"method-item-container\">", id).as_str();
        method_str += "\n";
        if let Some(name) = &method.name {
            method_str += format!("<h3 class=\"heading3\">{}</h3>", name).as_str();
            *toc += HtmlHelper::gen_url(format!("#{}", id).as_str(), name.as_str()).as_str();
        } else {
            method_str += r#"<h3 class="heading3">MISSING_NAME</h3>"#;
            *toc += HtmlHelper::gen_url(format!("#{}", id).as_str(), id.as_str()).as_str();
        }
        method_str += "\n";

        if let Some(signature) = &method.signature {
            let mut source = String::new();
            if let Some(ufunction) = &method.ufunction {
                source += ufunction.as_str();
                source.push('\n');
            }
            source += HtmlHelper::preprocess_source(signature).as_str();
            method_str += format!(
                "<pre class=\"method-signature-container\">\n<code>{}</code>\n</pre>",
                source
            )
            .as_str();
        }

        if let Some(desc) = &method.desc {
            method_str += r#"<div class="method-desc-container">"#;
            method_str += "\n";

            method_str +=
                HtmlHelper::preprocess_desc("method-desc", desc.description.as_str()).as_str();
            method_str += r#"</div>"#;
            method_str += "\n";
        }

        if let Some(returns) = &method.returns {
            method_str += "<h4 class=\"heading4\">returns</h4>\n";
            if let Some(returns_desc) = &returns.desc {
                if returns_desc.description.len() > 0 {
                    method_str += HtmlHelper::preprocess_desc(
                        "method-desc",
                        returns_desc.description.as_str(),
                    )
                    .as_str();
                } else {
                    method_str +=
                        HtmlHelper::preprocess_desc("method-desc", "MISSING_RET_DESC").as_str();
                }
            } else {
                method_str +=
                    HtmlHelper::preprocess_desc("method-desc", "MISSING_RET_DESC").as_str();
            }
        }

        if method.params.len() > 0 {
            method_str += "<h4 class=\"heading4\">params</h4>\n";
            method_str += "<center><table><tbody>\n";
            method_str += r#"<thead>
    <tr>
        <th>Parameter Name</th>
        <th style="width:80%">Description</th>
    </tr>
</thead>"#;
            for param in &method.params {
                method_str += "<tr>\n";

                // param name
                method_str += "<td>";
                if let Some(param_name) = &param.name {
                    method_str += param_name.as_str();
                } else {
                    method_str += "MISSING_PARAM_NAME";
                }
                method_str += "</td>\n";

                // param desc
                method_str += "<td>";
                if let Some(param_desc) = &param.desc {
                    if param_desc.description.as_str().len() == 0 {
                        method_str += "MISSING_PARAM_DESC";
                    } else {
                        method_str +=
                            HtmlHelper::preprocess_desc("", param_desc.description.as_str())
                                .as_str();
                    }
                } else {
                    // never enter?
                    method_str += "MISSING_PARAM_DESC";
                }
                method_str += "</td>\n";

                method_str += "</tr>\n";
            }
            method_str += "</tbody></table></center>\n";
        }

        method_str += "</div>\n";

        (method_str, id)
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

    fn preprocess_desc(class_name: &str, text: &str) -> String {
        let desc_splited_by_newline = text.to_string();
        let desc_splited_by_newline = desc_splited_by_newline.split("\n");
        let mut ret = String::new();
        for line in desc_splited_by_newline {
            ret += format!("<p class=\"{}\">{}</p>\n", class_name, line).as_str();
        }
        ret
    }
}
