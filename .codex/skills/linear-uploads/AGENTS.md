# Linear Uploads

Download attachments and images from Linear issues using `linear-cli`.

## Download to File

```bash
# Download image/attachment to file
linear-cli up fetch "https://uploads.linear.app/..." -f image.png

# Download screenshot
linear-cli up fetch "https://uploads.linear.app/abc/def/screenshot.png" -f /tmp/screenshot.png
```

## Output to Stdout

```bash
# Pipe to other tools
linear-cli up fetch "https://uploads.linear.app/..." | base64

# Pipe to file
linear-cli up fetch "https://uploads.linear.app/..." > file.png
```

## Finding Upload URLs

Upload URLs are found in:
- Issue descriptions
- Comments (use `linear-cli cm list ISSUE_ID --output json`)

URLs follow pattern: `https://uploads.linear.app/{org}/{upload}/{filename}`

## When to Use Each Mode

**Use `-f` (file) when the agent needs to view the image:**
```bash
# Download to /tmp, then read the file to view
linear-cli up fetch "https://uploads.linear.app/..." -f /tmp/screenshot.png
# Then read /tmp/screenshot.png (multimodal agents can view images)
```

**Use stdout when piping to other CLI tools:**
```bash
# Pipe to base64, imagemagick, etc.
linear-cli up fetch "https://uploads.linear.app/..." | base64
```

## Tips

- Requires valid Linear API key
- Use `-f` to specify output filename
- Without `-f`, outputs raw bytes to stdout
- Agents cannot interpret raw stdout bytes—always use `-f` + file read to view images
