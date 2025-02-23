pub fn view(children: String) -> String {
	let content = format!(
		r#"
			<!DOCTYPE html>

			<html lang="en">
				<head>
					<meta charset="UTF-8">
					<title>Akaia</title>
				</head>

				<body>
					{children}
				</body>
			</html>
		"#
	);

	content.to_string()
}
