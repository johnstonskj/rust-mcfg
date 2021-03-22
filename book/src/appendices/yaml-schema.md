# Appendix: Schema for YAML

## Schema for installer registry

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://github/schema/installers.json",
  "title": "Installer Registry",
  "description": "The Installer Registry file for mcfg",
  "definitions": {
    "name": {
      "$id": "#name",
      "type": "string",
      "pattern": "^[a-zA-Z0-9\\-+.@_/]+$"
    },
    "platform": {
      "$id": "#platform-kind",
      "type": "string",
      "enum": ["linux", "macos"]
    },
    "kind": {
      "$id": "#package-kind",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "application",
            "default",
            "language"
          ]
        },
        {
          "type": "object",
          "properties": {
            "language": {
              "type": "string",
              "pattern": "^[a-zA-Z0-9\\-+.@_/]+$"
            }
          },
          "required": [
            "language"
          ]
        }
      ]
    }
  },
  "type": "array",
  "items": {
    "type": "object",
    "properties": {
      "name": {
        "$ref": "#name"
      },
      "platform": {
        "$ref": "#platform-kind"
      },
      "kind": {
        "$ref": "#package-kind"
      },
      "if_exist": {
        "type": "string"
      },
      "commands": {
        "type": "object",
        "properties": {
          "install": {
            "type": "string"
          },
          "link-files": {
            "type": "string"
          },
          "uninstall": {
            "type": "string"
          },
          "update": {
            "type": "string"
          }
        }
      },
      "update-self": {
        "type": "string"
      }
    },
    "required": ["name"]
  }
}
```

## Schema for package sets

```json
{
  "$schema": "https://json-schema.org/draft/2020-12/schema",
  "$id": "https://github/schema/package-set.json",
  "title": "Package Set",
  "description": "A Package Set for mcfg",
  "definitions": {
    "name": {
      "$id": "#name",
      "type": "string",
      "pattern": "^[a-zA-Z0-9\\-+.@_/]+$"
    },
    "platform": {
      "$id": "#platform-kind",
      "type": "string",
      "enum": ["linux", "macos"]
    },
    "kind": {
      "$id": "#package-kind",
      "oneOf": [
        {
          "type": "string",
          "enum": [
            "application",
            "default",
            "language"
          ]
        },
        {
          "type": "object",
          "properties": {
            "language": {
              "type": "string",
              "pattern": "^[a-zA-Z0-9\\-+.@_/]+$"
            }
          },
          "required": [
            "language"
          ]
        }
      ]
    },
    "packages": {
      "$id": "#package-action",
      "type": "array",
      "items": {
        "type": "object",
        "properties": {
          "name": {
            "$ref": "#name"
          },
          "platform": {
            "$ref": "#platform-kind"
          },
          "kind": {
            "$ref": "#package-kind"
          }
        },
        "required": [
          "name"
        ]
      }
    },
    "scripts": {
      "$id": "#script-action",
      "type": "object",
      "properties": {
        "install": {
          "type": "string"
        },
        "link-files": {
          "type": "string"
        },
        "uninstall": {
          "type": "string"
        },
        "update": {
          "type": "string"
        }
      }
    }
  },
  "type": "object",
  "properties": {
    "name": {
      "$ref": "#name"
    },
    "description": { "type":  "string" },
    "platform": { "$ref":  "#platform-kind" },
    "optional": { "type": "boolean" },
    "env-vars": { "type": "object" },
    "run-before": { "type":  "string" },
    "run-after": { "type":  "string" },
    "link-files": { "type": "object" },
    "env-file": { "type":  "string" },
    "actions": {
      "type": "object",
      "oneOf": [
        {
          "$ref": "#package-action"
        },
        { "$ref":  "#script-action"}
      ]

    }
  },
  "required": ["name"]
}
```
