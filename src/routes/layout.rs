use crate::PageProps;

pub fn view(props: PageProps) -> String {
	let content = format!(
		r#"
			<!DOCTYPE html>

			<html lang="en">
				<head>
					<meta charset="UTF-8">
					<title>{title}</title>
					<meta name="description" content="{description}">
					<meta name="viewport" content="width=device-width, initial-scale=1.0">

					<style>
						{style}
					</style>
				</head>

				<body>
					{body}
				</body>
			</html>
		"#,
		title = format!(
			"{title} - {subtitle}",
			title = props.title,
			subtitle = props.subtitle
		),
		description = props.description,
		style = include_str!("../layout/style.css"),
		body = props.children
	);

	content.to_string()
}
