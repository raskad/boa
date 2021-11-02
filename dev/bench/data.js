window.BENCHMARK_DATA = {
  "lastUpdate": 1635813742858,
  "repoUrl": "https://github.com/raskad/boa",
  "entries": {
    "Boa Benchmarks": [
      {
        "commit": {
          "author": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "committer": {
            "email": "49699333+dependabot[bot]@users.noreply.github.com",
            "name": "dependabot[bot]",
            "username": "dependabot[bot]"
          },
          "distinct": true,
          "id": "09ba9b96b6fbf5f54ae9297025f43870675e9d82",
          "message": "Bump webpack from 5.60.0 to 5.61.0 (#1696)\n\nBumps [webpack](https://github.com/webpack/webpack) from 5.60.0 to 5.61.0.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a href=\"https://github.com/webpack/webpack/releases\">webpack's releases</a>.</em></p>\n<blockquote>\n<h2>v5.61.0</h2>\n<h1>Bugfixes</h1>\n<ul>\n<li>use a wasm md4 implementation for node 17 support</li>\n<li>include the <code>path</code> submodules in the node.js default externals</li>\n</ul>\n<h1>Performance</h1>\n<ul>\n<li>improve string to binary conversion performance for hashing</li>\n</ul>\n<h1>Contribution</h1>\n<ul>\n<li>CI runs on node.js 17</li>\n</ul>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a href=\"https://github.com/webpack/webpack/commit/0306510f7d1b9b13900f1dcf78a767c2ed390a61\"><code>0306510</code></a> 5.61.0</li>\n<li><a href=\"https://github.com/webpack/webpack/commit/42b4ffb01aa91abc48db16603976cae2009adb58\"><code>42b4ffb</code></a> Merge pull request <a href=\"https://github-redirect.dependabot.com/webpack/webpack/issues/14594\">#14594</a> from webpack/ci/node-17</li>\n<li><a href=\"https://github.com/webpack/webpack/commit/c2b6f7866e33c63288261c012b05e00aa6bacca2\"><code>c2b6f78</code></a> fix test cases for node 17</li>\n<li><a href=\"https://github.com/webpack/webpack/commit/1d7f6da4162f23995c8a5baf88ba9f5f73b33c76\"><code>1d7f6da</code></a> run CI on node 17</li>\n<li><a href=\"https://github.com/webpack/webpack/commit/1992e9ce2b739ec2864856f29a7cdf6134ab0de7\"><code>1992e9c</code></a> Merge pull request <a href=\"https://github-redirect.dependabot.com/webpack/webpack/issues/14583\">#14583</a> from juanrgm/fix/path-submodules-14582</li>\n<li><a href=\"https://github.com/webpack/webpack/commit/0f6c78cca174a73184fdc0d9c9c2bd376b48557c\"><code>0f6c78c</code></a> Merge pull request <a href=\"https://github-redirect.dependabot.com/webpack/webpack/issues/14584\">#14584</a> from webpack/hash/md4</li>\n<li><a href=\"https://github.com/webpack/webpack/commit/a6bb3e58ecba1d0468d65364ba629d5a04805d14\"><code>a6bb3e5</code></a> default to the default hash function</li>\n<li><a href=\"https://github.com/webpack/webpack/commit/205d3a05ee2a1cce4a4b8d7c270beaee784cfe78\"><code>205d3a0</code></a> add support for additional digest types</li>\n<li><a href=\"https://github.com/webpack/webpack/commit/d806cf5294a010f5c3280a38ab7500c681ca7c11\"><code>d806cf5</code></a> use correct batch size for BatchedHash</li>\n<li><a href=\"https://github.com/webpack/webpack/commit/7afc8330921f2c112d015e462910950c55aa9863\"><code>7afc833</code></a> improve micro benchmarks</li>\n<li>Additional commits viewable in <a href=\"https://github.com/webpack/webpack/compare/v5.60.0...v5.61.0\">compare view</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=webpack&package-manager=npm_and_yarn&previous-version=5.60.0&new-version=5.61.0)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits that have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after your CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge and block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually\n- `@dependabot ignore this major version` will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)\n\n\n</details>",
          "timestamp": "2021-11-01T17:07:25Z",
          "tree_id": "1cb2107c678ca32d65eb11b73ebf0931ca3d3d99",
          "url": "https://github.com/raskad/boa/commit/09ba9b96b6fbf5f54ae9297025f43870675e9d82"
        },
        "date": 1635813738481,
        "tool": "criterion",
        "benches": [
          {
            "name": "Create Realm",
            "value": 402.35,
            "range": "+/- 8.220",
            "unit": "ns"
          },
          {
            "name": "Symbols (Execution)",
            "value": 5.7098,
            "range": "+/- 0.152",
            "unit": "us"
          },
          {
            "name": "For loop (Execution)",
            "value": 23.783,
            "range": "+/- 0.452",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 3.1726,
            "range": "+/- 0.059",
            "unit": "ms"
          },
          {
            "name": "",
            "value": 8.9649,
            "range": "+/- 0.147",
            "unit": "us"
          },
          {
            "name": "",
            "value": 2.9076,
            "range": "+/- 0.060",
            "unit": "ms"
          },
          {
            "name": "Array pop (Execution)",
            "value": 939.62,
            "range": "+/- 22.240",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.8598,
            "range": "+/- 0.120",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.9969,
            "range": "+/- 0.126",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.4938,
            "range": "+/- 0.137",
            "unit": "us"
          },
          {
            "name": "",
            "value": 11.763,
            "range": "+/- 0.246",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution)",
            "value": 11.864,
            "range": "+/- 0.262",
            "unit": "us"
          },
          {
            "name": "",
            "value": 15.704,
            "range": "+/- 0.290",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution) #2",
            "value": 16.083,
            "range": "+/- 0.324",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.3687,
            "range": "+/- 0.146",
            "unit": "us"
          },
          {
            "name": "",
            "value": 8.9397,
            "range": "+/- 0.170",
            "unit": "us"
          },
          {
            "name": "String copy (Execution)",
            "value": 6.1607,
            "range": "+/- 0.131",
            "unit": "us"
          },
          {
            "name": "",
            "value": 4.304,
            "range": "+/- 0.112",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.4224,
            "range": "+/- 0.125",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.8784,
            "range": "+/- 0.147",
            "unit": "us"
          },
          {
            "name": "",
            "value": 262.43,
            "range": "+/- 5.500",
            "unit": "ns"
          },
          {
            "name": "Clean js (Execution)",
            "value": 795.45,
            "range": "+/- 12.160",
            "unit": "us"
          },
          {
            "name": "Mini js (Execution)",
            "value": 728.54,
            "range": "+/- 19.620",
            "unit": "us"
          },
          {
            "name": "Symbols (Full)",
            "value": 439.61,
            "range": "+/- 8.430",
            "unit": "us"
          },
          {
            "name": "For loop (Full)",
            "value": 486.1,
            "range": "+/- 8.490",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Full)",
            "value": 3.5785,
            "range": "+/- 0.053",
            "unit": "ms"
          },
          {
            "name": "Array access (Full)",
            "value": 479.29,
            "range": "+/- 7.880",
            "unit": "us"
          },
          {
            "name": "Array creation (Full)",
            "value": 3.4082,
            "range": "+/- 0.058",
            "unit": "ms"
          },
          {
            "name": "Array pop (Full)",
            "value": 1.5702,
            "range": "+/- 0.029",
            "unit": "ms"
          },
          {
            "name": "Object Creation (Full)",
            "value": 476.74,
            "range": "+/- 11.520",
            "unit": "us"
          },
          {
            "name": "",
            "value": 473.89,
            "range": "+/- 7.930",
            "unit": "us"
          },
          {
            "name": "",
            "value": 469.99,
            "range": "+/- 7.590",
            "unit": "us"
          },
          {
            "name": "",
            "value": 474.26,
            "range": "+/- 6.470",
            "unit": "us"
          },
          {
            "name": "RegExp (Full)",
            "value": 478.26,
            "range": "+/- 8.050",
            "unit": "us"
          },
          {
            "name": "RegExp Literal (Full)",
            "value": 490.19,
            "range": "+/- 9.790",
            "unit": "us"
          },
          {
            "name": "RegExp (Full) #2",
            "value": 494.92,
            "range": "+/- 10.300",
            "unit": "us"
          },
          {
            "name": "",
            "value": 475.19,
            "range": "+/- 12.100",
            "unit": "us"
          },
          {
            "name": "",
            "value": 475.35,
            "range": "+/- 7.930",
            "unit": "us"
          },
          {
            "name": "String copy (Full)",
            "value": 461.06,
            "range": "+/- 6.310",
            "unit": "us"
          },
          {
            "name": "",
            "value": 459.83,
            "range": "+/- 7.390",
            "unit": "us"
          },
          {
            "name": "",
            "value": 472.96,
            "range": "+/- 11.520",
            "unit": "us"
          },
          {
            "name": "",
            "value": 475.4,
            "range": "+/- 11.050",
            "unit": "us"
          },
          {
            "name": "",
            "value": 441.21,
            "range": "+/- 7.610",
            "unit": "us"
          },
          {
            "name": "Clean js (Full)",
            "value": 1.3122,
            "range": "+/- 0.031",
            "unit": "ms"
          },
          {
            "name": "Mini js (Full)",
            "value": 1.2181,
            "range": "+/- 0.026",
            "unit": "ms"
          },
          {
            "name": "Expression (Parser)",
            "value": 6.0908,
            "range": "+/- 0.151",
            "unit": "us"
          },
          {
            "name": "Hello World (Parser)",
            "value": 3.5434,
            "range": "+/- 0.066",
            "unit": "us"
          },
          {
            "name": "For loop (Parser)",
            "value": 17.067,
            "range": "+/- 0.318",
            "unit": "us"
          },
          {
            "name": "Long file (Parser)",
            "value": 809.15,
            "range": "+/- 16.920",
            "unit": "ns"
          },
          {
            "name": "Goal Symbols (Parser)",
            "value": 12.775,
            "range": "+/- 0.201",
            "unit": "us"
          },
          {
            "name": "Clean js (Parser)",
            "value": 36.049,
            "range": "+/- 0.601",
            "unit": "us"
          },
          {
            "name": "Mini js (Parser)",
            "value": 32.617,
            "range": "+/- 1.063",
            "unit": "us"
          }
        ]
      }
    ]
  }
}