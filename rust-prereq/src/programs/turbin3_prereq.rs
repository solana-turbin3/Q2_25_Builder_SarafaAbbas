use solana_idlgen::idlgen;

idlgen!(
  {
    "version": "0.1.0",
    "name": "turbine_prereq",
    "spec": "0.1.0",
    "description": "Created with Anchor",
  "instructions": [
    {
      "name": "complete",
      "discriminator": [
        0,
        77,
        224,
        147,
        136,
        25,
        88,
        76
      ],
      "accounts": [{
          "name": "signer",
          "isMut": true,
          "isSigner": true
        },
        {
          "name": "prereq",
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "system_program",
          "isMut": false,
          "isSigner": false
        }],
      "args": [
        {
          "name": "github",
          "type": "bytes"
        }]
    },
    {
      "name": "update",
      "discriminator": [
        219,
        200,
        88,
        176,
        158,
        63,
        253,
        127
      ],
      "accounts": [
        {
          "name": "signer",
          "writable": true,
          "signer": true,
          "isMut": true,
          "isSigner": true  // <-- Add isSigner here
        },
        {
          "name": "prereq",
          "writable": true,
          "isMut": true,
          "isSigner": false
        },
        {
          "name": "system_program",
          "address": "11111111111111111111111111111111",
          "isMut": false,
          "isSigner": false
        }
      ],
      "args": [
        {
          "name": "github",
          "type": "bytes"
        }
      ]
    }
  ],
  "errors": [
    {
      "code": 6000,
      "name": "InvalidGithubAccount",
      "msg": "Invalid Github account"
    }
  ],
  "types": [
    {
      "name": "SolanaCohort5Account",
      "type": {
        "kind": "struct",
        "fields": [
          {
            "name": "github",
            "type": "bytes"
          },
          {
            "name": "key",
            "type": "pubkey"
          }
        ]
      }
    }
  ],
  "metadata": {
    "address": "ADcaide4vBtKuyZQqdU689YqEGZMCmS4tL35bdTv9wJa"
  }
});