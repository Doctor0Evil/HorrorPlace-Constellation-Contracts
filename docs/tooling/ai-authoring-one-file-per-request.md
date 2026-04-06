# AI Authoring One-File-Per-Request

AI authoring flows MUST follow a one-file-per-request pattern.

An AI tool:

1. Receives an `ai-authoring-request.v1` object.
2. Produces exactly one `ai-authoring-response.v1` object.
3. Ensures the `payload` validates against the `schemaRef`.

External platforms can implement this contract to integrate safely with HorrorPlace repos.
