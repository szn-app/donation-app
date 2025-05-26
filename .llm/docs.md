# LLM refine prompts: 
- Claude generates detailed plan in markdown. 
- ChatGPT refine instructions. 
- Cursor composer agent.

# LLM issues: 
- [agentic mode] doesn't stop or wait when instructions indicate so. 
- [agentic mode] no format to run workflow in steps and ask for user confirmation before proceeding.
- [agentic mode] modifications of files which follow other files structure without referencing them often leads to inconsistencies and application of LLM recommended naming conventions (diverging model implementation from the database schema).