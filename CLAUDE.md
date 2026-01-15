## Linear Integration

Use `linear-cli` for all Linear.app operations. Do NOT use Linear MCP tools - CLI is 10-50x more token-efficient.

### Quick Commands

| Task | Command |
|------|---------|
| List issues | `linear-cli i list` |
| Create issue | `linear-cli i create "Title" -t TEAM -p 2` |
| View issue | `linear-cli i get LIN-123` |
| Start work | `linear-cli i start LIN-123 --checkout` |
| Update status | `linear-cli i update LIN-123 -s Done` |
| Create PR | `linear-cli g pr LIN-123` |
| Search | `linear-cli s issues "query"` |
| Get comments | `linear-cli cm list ISSUE_ID --output json` |
| Download upload | `linear-cli up fetch URL -f file.png` |

### Tips
- Add `--output json` to any command for machine-readable output
- Use short aliases: `i` (issues), `p` (projects), `g` (git), `s` (search), `cm` (comments)
- Run `linear-cli <command> --help` for full options
