window.BENCHMARK_DATA = {
  "lastUpdate": 1642952070440,
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
      },
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
          "id": "db520ebc46321f609ebdc93e54e9fe68eb0be811",
          "message": "Bump copy-webpack-plugin from 9.0.1 to 9.1.0 (#1709)\n\nBumps [copy-webpack-plugin](https://github.com/webpack-contrib/copy-webpack-plugin) from 9.0.1 to 9.1.0.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/releases\">copy-webpack-plugin's releases</a>.</em></p>\n<blockquote>\n<h2>v9.1.0</h2>\n<h2><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/compare/v9.0.1...v9.1.0\">9.1.0</a> (2021-11-11)</h2>\n<h3>Features</h3>\n<ul>\n<li>output helpful descriptions and links on errors (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/625\">#625</a>) (<a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/396bed6a8ad12cea344e988fefb9a554bb9c7b1a\">396bed6</a>)</li>\n</ul>\n<h3>Bug Fixes</h3>\n<ul>\n<li>compatibility with Node.js 17 (<a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/20af0c7f1b06a7e597e6b498dbc2b432a4a6d0de\">20af0c7</a>)</li>\n</ul>\n</blockquote>\n</details>\n<details>\n<summary>Changelog</summary>\n<p><em>Sourced from <a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/blob/master/CHANGELOG.md\">copy-webpack-plugin's changelog</a>.</em></p>\n<blockquote>\n<h2><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/compare/v9.0.1...v9.1.0\">9.1.0</a> (2021-11-11)</h2>\n<h3>Features</h3>\n<ul>\n<li>output helpful descriptions and links on errors (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/625\">#625</a>) (<a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/396bed6a8ad12cea344e988fefb9a554bb9c7b1a\">396bed6</a>)</li>\n</ul>\n<h3>Bug Fixes</h3>\n<ul>\n<li>compatibility with Node.js 17 (<a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/20af0c7f1b06a7e597e6b498dbc2b432a4a6d0de\">20af0c7</a>)</li>\n</ul>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/fbea73d5f6c0242c537dbb048c065ccdb9be54bf\"><code>fbea73d</code></a> chore(release): 9.1.0</li>\n<li><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/73bce86adba080dea1cfa08fa4384fd093f3e2d5\"><code>73bce86</code></a> refactor: reduce deps (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/639\">#639</a>)</li>\n<li><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/20af0c7f1b06a7e597e6b498dbc2b432a4a6d0de\"><code>20af0c7</code></a> fix: compatibility with Node.js 17</li>\n<li><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/fcab69bc68c113c14ac4e8bf8b38cb457c465578\"><code>fcab69b</code></a> chore(deps): update (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/637\">#637</a>)</li>\n<li><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/cc5af2f4321b606d8ac1e0de4f3384a0d144ab6b\"><code>cc5af2f</code></a> chore: fix typo (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/636\">#636</a>)</li>\n<li><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/5ddd582a318b635a0f187f572cf2f3ed9185eb02\"><code>5ddd582</code></a> docs: update bug report template (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/624\">#624</a>)</li>\n<li><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/396bed6a8ad12cea344e988fefb9a554bb9c7b1a\"><code>396bed6</code></a> feat: output helpful descriptions and links on errors (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/625\">#625</a>)</li>\n<li><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/6669609c1c70f591adb8f0c74b350e21a2496108\"><code>6669609</code></a> docs: update README (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/626\">#626</a>)</li>\n<li><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/b8c08da103b364695d00c33869945a16a1d3012b\"><code>b8c08da</code></a> chore: fix lint (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/622\">#622</a>)</li>\n<li><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/67cdcf11a55240475103fc611d32a8b06f1cf7b6\"><code>67cdcf1</code></a> docs: add options table (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/620\">#620</a>)</li>\n<li>Additional commits viewable in <a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/compare/v9.0.1...v9.1.0\">compare view</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=copy-webpack-plugin&package-manager=npm_and_yarn&previous-version=9.0.1&new-version=9.1.0)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits that have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after your CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge and block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually\n- `@dependabot ignore this major version` will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)\n\n\n</details>",
          "timestamp": "2021-11-12T16:32:02Z",
          "tree_id": "188e742f05fc8806e902f8ed36c30380034da0fe",
          "url": "https://github.com/raskad/boa/commit/db520ebc46321f609ebdc93e54e9fe68eb0be811"
        },
        "date": 1636856128528,
        "tool": "criterion",
        "benches": [
          {
            "name": "Create Realm",
            "value": 343.06,
            "range": "+/- 0.030",
            "unit": "ns"
          },
          {
            "name": "Symbols (Execution)",
            "value": 4.2337,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "For loop (Execution)",
            "value": 18.463,
            "range": "+/- 0.010",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 2.2312,
            "range": "+/- 0.003",
            "unit": "ms"
          },
          {
            "name": "",
            "value": 6.83,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "",
            "value": 2.5601,
            "range": "+/- 0.002",
            "unit": "ms"
          },
          {
            "name": "Array pop (Execution)",
            "value": 811.62,
            "range": "+/- 0.390",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.2258,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.4403,
            "range": "+/- 0.009",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.8602,
            "range": "+/- 0.007",
            "unit": "us"
          },
          {
            "name": "",
            "value": 9.2081,
            "range": "+/- 0.008",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution)",
            "value": 8.149,
            "range": "+/- 0.006",
            "unit": "us"
          },
          {
            "name": "",
            "value": 10.844,
            "range": "+/- 0.010",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution) #2",
            "value": 12.233,
            "range": "+/- 0.010",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.6953,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.1213,
            "range": "+/- 0.006",
            "unit": "us"
          },
          {
            "name": "String copy (Execution)",
            "value": 4.1769,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "",
            "value": 2.9415,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 4.178,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.5331,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "",
            "value": 199.88,
            "range": "+/- 0.100",
            "unit": "ns"
          },
          {
            "name": "Clean js (Execution)",
            "value": 593.96,
            "range": "+/- 0.640",
            "unit": "us"
          },
          {
            "name": "Mini js (Execution)",
            "value": 545.8,
            "range": "+/- 1.000",
            "unit": "us"
          },
          {
            "name": "Symbols (Full)",
            "value": 343.18,
            "range": "+/- 0.280",
            "unit": "us"
          },
          {
            "name": "For loop (Full)",
            "value": 377.78,
            "range": "+/- 0.250",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Full)",
            "value": 2.6843,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "Array access (Full)",
            "value": 373.34,
            "range": "+/- 0.310",
            "unit": "us"
          },
          {
            "name": "Array creation (Full)",
            "value": 2.9948,
            "range": "+/- 0.007",
            "unit": "ms"
          },
          {
            "name": "Array pop (Full)",
            "value": 1.3515,
            "range": "+/- 0.000",
            "unit": "ms"
          },
          {
            "name": "Object Creation (Full)",
            "value": 322.5,
            "range": "+/- 0.250",
            "unit": "us"
          },
          {
            "name": "",
            "value": 324.6,
            "range": "+/- 0.520",
            "unit": "us"
          },
          {
            "name": "",
            "value": 367.35,
            "range": "+/- 0.280",
            "unit": "us"
          },
          {
            "name": "",
            "value": 367.4,
            "range": "+/- 0.470",
            "unit": "us"
          },
          {
            "name": "RegExp (Full)",
            "value": 364.9,
            "range": "+/- 0.150",
            "unit": "us"
          },
          {
            "name": "RegExp Literal (Full)",
            "value": 376.19,
            "range": "+/- 0.150",
            "unit": "us"
          },
          {
            "name": "RegExp (Full) #2",
            "value": 382.51,
            "range": "+/- 0.270",
            "unit": "us"
          },
          {
            "name": "",
            "value": 359.52,
            "range": "+/- 0.160",
            "unit": "us"
          },
          {
            "name": "",
            "value": 359.82,
            "range": "+/- 0.180",
            "unit": "us"
          },
          {
            "name": "String copy (Full)",
            "value": 351.39,
            "range": "+/- 0.260",
            "unit": "us"
          },
          {
            "name": "",
            "value": 351.94,
            "range": "+/- 0.140",
            "unit": "us"
          },
          {
            "name": "",
            "value": 354.34,
            "range": "+/- 0.230",
            "unit": "us"
          },
          {
            "name": "",
            "value": 360.3,
            "range": "+/- 0.290",
            "unit": "us"
          },
          {
            "name": "",
            "value": 335.97,
            "range": "+/- 0.150",
            "unit": "us"
          },
          {
            "name": "Clean js (Full)",
            "value": 1.0688,
            "range": "+/- 0.002",
            "unit": "ms"
          },
          {
            "name": "Mini js (Full)",
            "value": 1.01,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "Expression (Parser)",
            "value": 5.3316,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "Hello World (Parser)",
            "value": 3.0661,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "For loop (Parser)",
            "value": 15.062,
            "range": "+/- 0.046",
            "unit": "us"
          },
          {
            "name": "Long file (Parser)",
            "value": 732.5,
            "range": "+/- 0.210",
            "unit": "ns"
          },
          {
            "name": "Goal Symbols (Parser)",
            "value": 11.069,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "Clean js (Parser)",
            "value": 31.609,
            "range": "+/- 0.010",
            "unit": "us"
          },
          {
            "name": "Mini js (Parser)",
            "value": 27.665,
            "range": "+/- 0.009",
            "unit": "us"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "RageKnify@gmail.com",
            "name": "João Borges",
            "username": "RageKnify"
          },
          "committer": {
            "email": "RageKnify@gmail.com",
            "name": "João Borges",
            "username": "RageKnify"
          },
          "distinct": true,
          "id": "49ae8441c472910addb16b05bbb06d981903ad83",
          "message": "refactor: fix construct_error functions (#1703)\n\nBefore the interpreter would create the AST equivalent to `new Error(message)` and interpret it when constructing the builtin errors, this could fail if the gloabl in question had been overwritten. Now we use `Context::standard_objects` to get access to the\r\nerror constructors and invoke the functions directly.",
          "timestamp": "2021-11-14T01:52:52Z",
          "tree_id": "e4ae8650d117991109a0fa90255766b98cd766a5",
          "url": "https://github.com/raskad/boa/commit/49ae8441c472910addb16b05bbb06d981903ad83"
        },
        "date": 1636856573934,
        "tool": "criterion",
        "benches": [
          {
            "name": "Create Realm",
            "value": 446.09,
            "range": "+/- 6.400",
            "unit": "ns"
          },
          {
            "name": "Symbols (Execution)",
            "value": 6.1451,
            "range": "+/- 0.162",
            "unit": "us"
          },
          {
            "name": "For loop (Execution)",
            "value": 25.599,
            "range": "+/- 0.454",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 3.3279,
            "range": "+/- 0.049",
            "unit": "ms"
          },
          {
            "name": "",
            "value": 9.584,
            "range": "+/- 0.172",
            "unit": "us"
          },
          {
            "name": "",
            "value": 3.1206,
            "range": "+/- 0.053",
            "unit": "ms"
          },
          {
            "name": "Array pop (Execution)",
            "value": 988.19,
            "range": "+/- -975.308",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.2769,
            "range": "+/- 0.168",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.4572,
            "range": "+/- 0.127",
            "unit": "us"
          },
          {
            "name": "",
            "value": 8.1873,
            "range": "+/- 0.189",
            "unit": "us"
          },
          {
            "name": "",
            "value": 12.779,
            "range": "+/- 0.318",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution)",
            "value": 12.366,
            "range": "+/- 0.294",
            "unit": "us"
          },
          {
            "name": "",
            "value": 17.329,
            "range": "+/- 0.593",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution) #2",
            "value": 17.438,
            "range": "+/- 0.392",
            "unit": "us"
          },
          {
            "name": "",
            "value": 8.0005,
            "range": "+/- 0.156",
            "unit": "us"
          },
          {
            "name": "",
            "value": 9.214,
            "range": "+/- 0.133",
            "unit": "us"
          },
          {
            "name": "String copy (Execution)",
            "value": 6.629,
            "range": "+/- 0.128",
            "unit": "us"
          },
          {
            "name": "",
            "value": 4.2783,
            "range": "+/- 0.073",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.9905,
            "range": "+/- 0.141",
            "unit": "us"
          },
          {
            "name": "",
            "value": 8.1524,
            "range": "+/- 0.135",
            "unit": "us"
          },
          {
            "name": "",
            "value": 305.74,
            "range": "+/- 5.920",
            "unit": "ns"
          },
          {
            "name": "Clean js (Execution)",
            "value": 834.92,
            "range": "+/- 20.540",
            "unit": "us"
          },
          {
            "name": "Mini js (Execution)",
            "value": 767.95,
            "range": "+/- 11.340",
            "unit": "us"
          },
          {
            "name": "Symbols (Full)",
            "value": 455.73,
            "range": "+/- 6.110",
            "unit": "us"
          },
          {
            "name": "For loop (Full)",
            "value": 493.62,
            "range": "+/- 10.260",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Full)",
            "value": 3.829,
            "range": "+/- 0.058",
            "unit": "ms"
          },
          {
            "name": "Array access (Full)",
            "value": 498.96,
            "range": "+/- 8.150",
            "unit": "us"
          },
          {
            "name": "Array creation (Full)",
            "value": 3.7652,
            "range": "+/- 0.053",
            "unit": "ms"
          },
          {
            "name": "Array pop (Full)",
            "value": 1.6883,
            "range": "+/- 0.027",
            "unit": "ms"
          },
          {
            "name": "Object Creation (Full)",
            "value": 473.15,
            "range": "+/- 5.770",
            "unit": "us"
          },
          {
            "name": "",
            "value": 496.99,
            "range": "+/- 15.220",
            "unit": "us"
          },
          {
            "name": "",
            "value": 499.88,
            "range": "+/- 10.130",
            "unit": "us"
          },
          {
            "name": "",
            "value": 497.32,
            "range": "+/- 9.530",
            "unit": "us"
          },
          {
            "name": "RegExp (Full)",
            "value": 486.81,
            "range": "+/- 11.160",
            "unit": "us"
          },
          {
            "name": "RegExp Literal (Full)",
            "value": 509.3,
            "range": "+/- 7.850",
            "unit": "us"
          },
          {
            "name": "RegExp (Full) #2",
            "value": 524.76,
            "range": "+/- 9.690",
            "unit": "us"
          },
          {
            "name": "",
            "value": 480.8,
            "range": "+/- 10.510",
            "unit": "us"
          },
          {
            "name": "",
            "value": 486.42,
            "range": "+/- 7.300",
            "unit": "us"
          },
          {
            "name": "String copy (Full)",
            "value": 461.14,
            "range": "+/- 6.190",
            "unit": "us"
          },
          {
            "name": "",
            "value": 472.33,
            "range": "+/- 7.220",
            "unit": "us"
          },
          {
            "name": "",
            "value": 483.17,
            "range": "+/- 9.990",
            "unit": "us"
          },
          {
            "name": "",
            "value": 476.33,
            "range": "+/- 7.940",
            "unit": "us"
          },
          {
            "name": "",
            "value": 452.87,
            "range": "+/- 8.250",
            "unit": "us"
          },
          {
            "name": "Clean js (Full)",
            "value": 1.4034,
            "range": "+/- 0.017",
            "unit": "ms"
          },
          {
            "name": "Mini js (Full)",
            "value": 1.3135,
            "range": "+/- 0.023",
            "unit": "ms"
          },
          {
            "name": "Expression (Parser)",
            "value": 6.6488,
            "range": "+/- 0.157",
            "unit": "us"
          },
          {
            "name": "Hello World (Parser)",
            "value": 3.6509,
            "range": "+/- 0.047",
            "unit": "us"
          },
          {
            "name": "For loop (Parser)",
            "value": 18.299,
            "range": "+/- 0.433",
            "unit": "us"
          },
          {
            "name": "Long file (Parser)",
            "value": 857.49,
            "range": "+/- 15.320",
            "unit": "ns"
          },
          {
            "name": "Goal Symbols (Parser)",
            "value": 13.526,
            "range": "+/- 0.381",
            "unit": "us"
          },
          {
            "name": "Clean js (Parser)",
            "value": 38.08,
            "range": "+/- 0.823",
            "unit": "us"
          },
          {
            "name": "Mini js (Parser)",
            "value": 32.74,
            "range": "+/- 0.526",
            "unit": "us"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "32105367+raskad@users.noreply.github.com",
            "name": "raskad",
            "username": "raskad"
          },
          "committer": {
            "email": "32105367+raskad@users.noreply.github.com",
            "name": "raskad",
            "username": "raskad"
          },
          "distinct": true,
          "id": "ca922310f2a868662f560588c0a0858ad475699c",
          "message": "Implement missing vm operations (#1697)\n\nThis implements most of the missing vm operations and fixes most of the panics.\r\n\r\nThe early errors for declarations in `for in` and `for of` loops are moved to the parser. The content of those `Node`s is changed accordingly.",
          "timestamp": "2021-11-16T22:33:34Z",
          "tree_id": "fd826fd513667a230215b1105f94907079368085",
          "url": "https://github.com/raskad/boa/commit/ca922310f2a868662f560588c0a0858ad475699c"
        },
        "date": 1637103962656,
        "tool": "criterion",
        "benches": [
          {
            "name": "Create Realm",
            "value": 429.65,
            "range": "+/- 0.770",
            "unit": "ns"
          },
          {
            "name": "Symbols (Execution)",
            "value": 5.2011,
            "range": "+/- 0.010",
            "unit": "us"
          },
          {
            "name": "For loop (Execution)",
            "value": 23.566,
            "range": "+/- 0.009",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 2.7835,
            "range": "+/- 0.002",
            "unit": "ms"
          },
          {
            "name": "",
            "value": 8.4912,
            "range": "+/- 0.007",
            "unit": "us"
          },
          {
            "name": "",
            "value": 3.3204,
            "range": "+/- 0.002",
            "unit": "ms"
          },
          {
            "name": "Array pop (Execution)",
            "value": 1.0528,
            "range": "+/- 0.000",
            "unit": "ms"
          },
          {
            "name": "",
            "value": 6.401,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.7149,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.3489,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 11.988,
            "range": "+/- 0.006",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution)",
            "value": 11.969,
            "range": "+/- 0.013",
            "unit": "us"
          },
          {
            "name": "",
            "value": 15.487,
            "range": "+/- 0.013",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution) #2",
            "value": 15.515,
            "range": "+/- 0.018",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.1501,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "",
            "value": 8.6991,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "String copy (Execution)",
            "value": 5.8981,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 3.9936,
            "range": "+/- 0.009",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.1802,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.7563,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "",
            "value": 276.53,
            "range": "+/- 0.080",
            "unit": "ns"
          },
          {
            "name": "Clean js (Execution)",
            "value": 843.24,
            "range": "+/- 1.100",
            "unit": "us"
          },
          {
            "name": "Mini js (Execution)",
            "value": 771.68,
            "range": "+/- 1.120",
            "unit": "us"
          },
          {
            "name": "Symbols (Full)",
            "value": 429.82,
            "range": "+/- 0.290",
            "unit": "us"
          },
          {
            "name": "For loop (Full)",
            "value": 472.83,
            "range": "+/- 0.360",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Full)",
            "value": 3.2892,
            "range": "+/- 0.004",
            "unit": "ms"
          },
          {
            "name": "Array access (Full)",
            "value": 462.35,
            "range": "+/- 0.650",
            "unit": "us"
          },
          {
            "name": "Array creation (Full)",
            "value": 4.0101,
            "range": "+/- 0.002",
            "unit": "ms"
          },
          {
            "name": "Array pop (Full)",
            "value": 1.7403,
            "range": "+/- 0.000",
            "unit": "ms"
          },
          {
            "name": "Object Creation (Full)",
            "value": 454.31,
            "range": "+/- 0.210",
            "unit": "us"
          },
          {
            "name": "",
            "value": 452.08,
            "range": "+/- 0.180",
            "unit": "us"
          },
          {
            "name": "",
            "value": 455.6,
            "range": "+/- 0.360",
            "unit": "us"
          },
          {
            "name": "",
            "value": 455.87,
            "range": "+/- 0.410",
            "unit": "us"
          },
          {
            "name": "RegExp (Full)",
            "value": 454.18,
            "range": "+/- 0.180",
            "unit": "us"
          },
          {
            "name": "RegExp Literal (Full)",
            "value": 468.91,
            "range": "+/- 0.350",
            "unit": "us"
          },
          {
            "name": "RegExp (Full) #2",
            "value": 470.59,
            "range": "+/- 0.220",
            "unit": "us"
          },
          {
            "name": "",
            "value": 447.4,
            "range": "+/- 0.340",
            "unit": "us"
          },
          {
            "name": "",
            "value": 454.36,
            "range": "+/- 0.690",
            "unit": "us"
          },
          {
            "name": "String copy (Full)",
            "value": 436.77,
            "range": "+/- 0.500",
            "unit": "us"
          },
          {
            "name": "",
            "value": 440.33,
            "range": "+/- 0.190",
            "unit": "us"
          },
          {
            "name": "",
            "value": 443.35,
            "range": "+/- 0.190",
            "unit": "us"
          },
          {
            "name": "",
            "value": 449.23,
            "range": "+/- 0.590",
            "unit": "us"
          },
          {
            "name": "",
            "value": 422.14,
            "range": "+/- 0.210",
            "unit": "us"
          },
          {
            "name": "Clean js (Full)",
            "value": 1.3446,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "Mini js (Full)",
            "value": 1.2749,
            "range": "+/- 0.002",
            "unit": "ms"
          },
          {
            "name": "Expression (Parser)",
            "value": 6.3763,
            "range": "+/- 0.023",
            "unit": "us"
          },
          {
            "name": "Hello World (Parser)",
            "value": 3.6592,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "For loop (Parser)",
            "value": 17.647,
            "range": "+/- 0.007",
            "unit": "us"
          },
          {
            "name": "Long file (Parser)",
            "value": 829.2,
            "range": "+/- 0.130",
            "unit": "ns"
          },
          {
            "name": "Goal Symbols (Parser)",
            "value": 13.084,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "Clean js (Parser)",
            "value": 36.509,
            "range": "+/- 0.027",
            "unit": "us"
          },
          {
            "name": "Mini js (Parser)",
            "value": 32.057,
            "range": "+/- 0.009",
            "unit": "us"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "halidodat@gmail.com",
            "name": "Halid Odat",
            "username": "HalidOdat"
          },
          "committer": {
            "email": "halidodat@gmail.com",
            "name": "Halid Odat",
            "username": "HalidOdat"
          },
          "distinct": true,
          "id": "3269c1b9016ff4503c8f935e81b580c3f8d20708",
          "message": "Generic `JsResult<R>` in `context.throw_` methods (#1734)\n\nPreviously when we had the `context.throw_` methods (like `context.thtrow_type_error()`) they were limited as to where we could call them, e.i. a function that returned `JsResult<JsValue>`. So we had to call the `context.construct_` methods with an explicit `Err()` enum wrap to throw in functions that returned non-jsvalues (which happens a lot).\r\nNow, with this PR the throw methods have a generic `JsResult<R>` return that can return in any `JsResult<T>` returning function. Which cleans the API and makes the user experience a bit better.\r\n\r\n```rust\r\nreturn Err(context.construct_type_error(\"...\"));\r\n// to\r\nreturn context.throw_type_error(\"...\");\r\n```",
          "timestamp": "2021-12-06T21:18:10Z",
          "tree_id": "795e69338ae90f406aa06b98164e72b0a4467640",
          "url": "https://github.com/raskad/boa/commit/3269c1b9016ff4503c8f935e81b580c3f8d20708"
        },
        "date": 1638829556616,
        "tool": "criterion",
        "benches": [
          {
            "name": "Create Realm",
            "value": 421.12,
            "range": "+/- 12.310",
            "unit": "ns"
          },
          {
            "name": "Symbols (Execution)",
            "value": 6.0569,
            "range": "+/- 0.131",
            "unit": "us"
          },
          {
            "name": "For loop (Execution)",
            "value": 25.079,
            "range": "+/- 0.757",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 3.078,
            "range": "+/- 0.052",
            "unit": "ms"
          },
          {
            "name": "",
            "value": 8.8534,
            "range": "+/- 0.269",
            "unit": "us"
          },
          {
            "name": "",
            "value": 2.9886,
            "range": "+/- 0.061",
            "unit": "ms"
          },
          {
            "name": "Array pop (Execution)",
            "value": 940.17,
            "range": "+/- 25.900",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.7484,
            "range": "+/- 0.155",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.4977,
            "range": "+/- 0.176",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.0365,
            "range": "+/- 0.190",
            "unit": "us"
          },
          {
            "name": "",
            "value": 10.772,
            "range": "+/- 0.437",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution)",
            "value": 12.192,
            "range": "+/- 0.415",
            "unit": "us"
          },
          {
            "name": "",
            "value": 16.435,
            "range": "+/- 0.401",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution) #2",
            "value": 16.567,
            "range": "+/- 0.451",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.4844,
            "range": "+/- 0.233",
            "unit": "us"
          },
          {
            "name": "",
            "value": 8.4402,
            "range": "+/- 0.231",
            "unit": "us"
          },
          {
            "name": "String copy (Execution)",
            "value": 5.8573,
            "range": "+/- 0.141",
            "unit": "us"
          },
          {
            "name": "",
            "value": 3.902,
            "range": "+/- 0.100",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.3893,
            "range": "+/- 0.144",
            "unit": "us"
          },
          {
            "name": "",
            "value": 8.3711,
            "range": "+/- 0.193",
            "unit": "us"
          },
          {
            "name": "",
            "value": 280.06,
            "range": "+/- 6.040",
            "unit": "ns"
          },
          {
            "name": "Clean js (Execution)",
            "value": 771.27,
            "range": "+/- 19.540",
            "unit": "us"
          },
          {
            "name": "Mini js (Execution)",
            "value": 753.25,
            "range": "+/- 12.230",
            "unit": "us"
          },
          {
            "name": "Symbols (Full)",
            "value": 447.01,
            "range": "+/- 5.380",
            "unit": "us"
          },
          {
            "name": "For loop (Full)",
            "value": 485.18,
            "range": "+/- 12.620",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Full)",
            "value": 3.5472,
            "range": "+/- 0.049",
            "unit": "ms"
          },
          {
            "name": "Array access (Full)",
            "value": 477.83,
            "range": "+/- 11.610",
            "unit": "us"
          },
          {
            "name": "Array creation (Full)",
            "value": 3.483,
            "range": "+/- 0.069",
            "unit": "ms"
          },
          {
            "name": "Array pop (Full)",
            "value": 1.626,
            "range": "+/- 0.035",
            "unit": "ms"
          },
          {
            "name": "Object Creation (Full)",
            "value": 457.17,
            "range": "+/- 12.040",
            "unit": "us"
          },
          {
            "name": "",
            "value": 468.38,
            "range": "+/- 11.150",
            "unit": "us"
          },
          {
            "name": "",
            "value": 452.8,
            "range": "+/- 10.620",
            "unit": "us"
          },
          {
            "name": "",
            "value": 475.37,
            "range": "+/- 13.990",
            "unit": "us"
          },
          {
            "name": "RegExp (Full)",
            "value": 476.53,
            "range": "+/- 10.740",
            "unit": "us"
          },
          {
            "name": "RegExp Literal (Full)",
            "value": 498.54,
            "range": "+/- 10.200",
            "unit": "us"
          },
          {
            "name": "RegExp (Full) #2",
            "value": 459.18,
            "range": "+/- 11.020",
            "unit": "us"
          },
          {
            "name": "",
            "value": 440.7,
            "range": "+/- 11.150",
            "unit": "us"
          },
          {
            "name": "",
            "value": 463.1,
            "range": "+/- 15.400",
            "unit": "us"
          },
          {
            "name": "String copy (Full)",
            "value": 446.25,
            "range": "+/- 9.790",
            "unit": "us"
          },
          {
            "name": "",
            "value": 464.05,
            "range": "+/- 10.090",
            "unit": "us"
          },
          {
            "name": "",
            "value": 474.73,
            "range": "+/- 7.410",
            "unit": "us"
          },
          {
            "name": "",
            "value": 466.2,
            "range": "+/- 11.180",
            "unit": "us"
          },
          {
            "name": "",
            "value": 447.55,
            "range": "+/- 9.480",
            "unit": "us"
          },
          {
            "name": "Clean js (Full)",
            "value": 1.3666,
            "range": "+/- 0.030",
            "unit": "ms"
          },
          {
            "name": "Mini js (Full)",
            "value": 1.3162,
            "range": "+/- 0.027",
            "unit": "ms"
          },
          {
            "name": "Expression (Parser)",
            "value": 6.4567,
            "range": "+/- 0.182",
            "unit": "us"
          },
          {
            "name": "Hello World (Parser)",
            "value": 3.6559,
            "range": "+/- 0.124",
            "unit": "us"
          },
          {
            "name": "For loop (Parser)",
            "value": 17.495,
            "range": "+/- 0.698",
            "unit": "us"
          },
          {
            "name": "Long file (Parser)",
            "value": 828.26,
            "range": "+/- 20.630",
            "unit": "ns"
          },
          {
            "name": "Goal Symbols (Parser)",
            "value": 13.593,
            "range": "+/- 0.283",
            "unit": "us"
          },
          {
            "name": "Clean js (Parser)",
            "value": 37.863,
            "range": "+/- 1.210",
            "unit": "us"
          },
          {
            "name": "Mini js (Parser)",
            "value": 33.825,
            "range": "+/- 0.632",
            "unit": "us"
          }
        ]
      },
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
          "id": "6cb93cfe1c7ad119129db74d2d6d535b94b529a1",
          "message": "Bump serde_yaml from 0.8.21 to 0.8.23 (#1740)\n\nBumps [serde_yaml](https://github.com/dtolnay/serde-yaml) from 0.8.21 to 0.8.23.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a href=\"https://github.com/dtolnay/serde-yaml/releases\">serde_yaml's releases</a>.</em></p>\n<blockquote>\n<h2>0.8.23</h2>\n<ul>\n<li>Fix handling of YAML 1.1-style octals that begin with <code>+</code> or <code>-</code> sign (<a href=\"https://github-redirect.dependabot.com/dtolnay/serde-yaml/issues/228\">#228</a>)</li>\n</ul>\n<h2>0.8.22</h2>\n<ul>\n<li>Switch float serializer to use the same float formatting library as serde_json</li>\n</ul>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a href=\"https://github.com/dtolnay/serde-yaml/commit/44ba87954d502651689ac1cc3026effa91785a95\"><code>44ba879</code></a> Release 0.8.23</li>\n<li><a href=\"https://github.com/dtolnay/serde-yaml/commit/f850da3d4ba2260fe9d991e684cf12d30157679c\"><code>f850da3</code></a> Merge pull request <a href=\"https://github-redirect.dependabot.com/dtolnay/serde-yaml/issues/228\">#228</a> from dtolnay/octal</li>\n<li><a href=\"https://github.com/dtolnay/serde-yaml/commit/983b5b3f5fd255225acd4cab07672608b86a642d\"><code>983b5b3</code></a> Suppress blocks_in_if_conditions clippy lint</li>\n<li><a href=\"https://github.com/dtolnay/serde-yaml/commit/57f2e661b8db22491b404f0e515c362bc6d8a235\"><code>57f2e66</code></a> Treat YAML 1.1 style octals with sign as string, not base 10 number</li>\n<li><a href=\"https://github.com/dtolnay/serde-yaml/commit/f424a15c74794c0c9f5834d976db81a45e00d300\"><code>f424a15</code></a> Add test of cases that are NOT supposed to be numbers</li>\n<li><a href=\"https://github.com/dtolnay/serde-yaml/commit/75a43c4dbc882cd8c588a35f27989667778dbe90\"><code>75a43c4</code></a> Merge pull request <a href=\"https://github-redirect.dependabot.com/dtolnay/serde-yaml/issues/218\">#218</a> from rukai/add_unit_struct_test</li>\n<li><a href=\"https://github.com/dtolnay/serde-yaml/commit/1db8fdbb8560ef2e63b3e9a1295b193f405c742b\"><code>1db8fdb</code></a> Release 0.8.22</li>\n<li><a href=\"https://github.com/dtolnay/serde-yaml/commit/6b836037b58ebb359e7c485fc6002b1e8214bd6c\"><code>6b83603</code></a> Switch float serializer from dtoa to ryu</li>\n<li><a href=\"https://github.com/dtolnay/serde-yaml/commit/7a02d7377051e3afc07c4db628d76fdc64e8b2c2\"><code>7a02d73</code></a> Delete disabled borrowed string deserializer test</li>\n<li><a href=\"https://github.com/dtolnay/serde-yaml/commit/eb31d62693057ca34e24a983354a96d145c8477f\"><code>eb31d62</code></a> Update from dtoa 0.4 to 1.0</li>\n<li>Additional commits viewable in <a href=\"https://github.com/dtolnay/serde-yaml/compare/0.8.21...0.8.23\">compare view</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=serde_yaml&package-manager=cargo&previous-version=0.8.21&new-version=0.8.23)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits that have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after your CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge and block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually\n- `@dependabot ignore this major version` will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)\n\n\n</details>",
          "timestamp": "2021-12-13T17:28:29Z",
          "tree_id": "70cf4ba927d3e50da03b17619885f26b4ed579e2",
          "url": "https://github.com/raskad/boa/commit/6cb93cfe1c7ad119129db74d2d6d535b94b529a1"
        },
        "date": 1639435329186,
        "tool": "criterion",
        "benches": [
          {
            "name": "Create Realm",
            "value": 347.34,
            "range": "+/- 0.500",
            "unit": "ns"
          },
          {
            "name": "Symbols (Execution)",
            "value": 4.2431,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "For loop (Execution)",
            "value": 18.052,
            "range": "+/- 0.011",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 2.256,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "",
            "value": 6.7018,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "",
            "value": 2.5069,
            "range": "+/- 0.002",
            "unit": "ms"
          },
          {
            "name": "Array pop (Execution)",
            "value": 804.01,
            "range": "+/- 0.330",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.1963,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.3775,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.8507,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "",
            "value": 9.5246,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution)",
            "value": 9.5267,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "",
            "value": 12.55,
            "range": "+/- 0.011",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution) #2",
            "value": 12.53,
            "range": "+/- 0.011",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.5746,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.9227,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "String copy (Execution)",
            "value": 4.6992,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "",
            "value": 3.2086,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 4.1934,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.1965,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 228.97,
            "range": "+/- 0.060",
            "unit": "ns"
          },
          {
            "name": "Clean js (Execution)",
            "value": 663.46,
            "range": "+/- 0.900",
            "unit": "us"
          },
          {
            "name": "Mini js (Execution)",
            "value": 609.34,
            "range": "+/- 0.900",
            "unit": "us"
          },
          {
            "name": "Symbols (Full)",
            "value": 342.98,
            "range": "+/- 0.190",
            "unit": "us"
          },
          {
            "name": "For loop (Full)",
            "value": 377.86,
            "range": "+/- 1.530",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Full)",
            "value": 2.6848,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "Array access (Full)",
            "value": 371.77,
            "range": "+/- 0.250",
            "unit": "us"
          },
          {
            "name": "Array creation (Full)",
            "value": 2.9309,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "Array pop (Full)",
            "value": 1.3312,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "Object Creation (Full)",
            "value": 363.43,
            "range": "+/- 0.200",
            "unit": "us"
          },
          {
            "name": "",
            "value": 358.34,
            "range": "+/- 0.150",
            "unit": "us"
          },
          {
            "name": "",
            "value": 364.74,
            "range": "+/- 0.910",
            "unit": "us"
          },
          {
            "name": "",
            "value": 364.27,
            "range": "+/- 0.220",
            "unit": "us"
          },
          {
            "name": "RegExp (Full)",
            "value": 367.76,
            "range": "+/- 1.290",
            "unit": "us"
          },
          {
            "name": "RegExp Literal (Full)",
            "value": 379.44,
            "range": "+/- 0.210",
            "unit": "us"
          },
          {
            "name": "RegExp (Full) #2",
            "value": 378.32,
            "range": "+/- 0.260",
            "unit": "us"
          },
          {
            "name": "",
            "value": 356.75,
            "range": "+/- 0.110",
            "unit": "us"
          },
          {
            "name": "",
            "value": 365.41,
            "range": "+/- 0.430",
            "unit": "us"
          },
          {
            "name": "String copy (Full)",
            "value": 354.08,
            "range": "+/- 0.210",
            "unit": "us"
          },
          {
            "name": "",
            "value": 353.89,
            "range": "+/- 0.150",
            "unit": "us"
          },
          {
            "name": "",
            "value": 356.63,
            "range": "+/- 0.200",
            "unit": "us"
          },
          {
            "name": "",
            "value": 361.11,
            "range": "+/- 0.160",
            "unit": "us"
          },
          {
            "name": "",
            "value": 340.76,
            "range": "+/- 0.250",
            "unit": "us"
          },
          {
            "name": "Clean js (Full)",
            "value": 1.0554,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "Mini js (Full)",
            "value": 996.08,
            "range": "+/- 1.330",
            "unit": "us"
          },
          {
            "name": "Expression (Parser)",
            "value": 5.2597,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "Hello World (Parser)",
            "value": 3.1716,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "For loop (Parser)",
            "value": 15.06,
            "range": "+/- 0.008",
            "unit": "us"
          },
          {
            "name": "Long file (Parser)",
            "value": 727.84,
            "range": "+/- 0.150",
            "unit": "ns"
          },
          {
            "name": "Goal Symbols (Parser)",
            "value": 11.19,
            "range": "+/- 0.007",
            "unit": "us"
          },
          {
            "name": "Clean js (Parser)",
            "value": 31.911,
            "range": "+/- 0.013",
            "unit": "us"
          },
          {
            "name": "Mini js (Parser)",
            "value": 28.065,
            "range": "+/- 0.021",
            "unit": "us"
          }
        ]
      },
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
          "id": "fed019d94fc84e115283ec4522f26c5ed9c5527b",
          "message": "Bump copy-webpack-plugin from 10.1.0 to 10.2.0 (#1749)\n\nBumps [copy-webpack-plugin](https://github.com/webpack-contrib/copy-webpack-plugin) from 10.1.0 to 10.2.0.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/releases\">copy-webpack-plugin's releases</a>.</em></p>\n<blockquote>\n<h2>v10.2.0</h2>\n<h2><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/compare/v10.1.0...v10.2.0\">10.2.0</a> (2021-12-16)</h2>\n<h3>Features</h3>\n<ul>\n<li>removed cjs wrapper and generated types in commonjs format (<code>export =</code> and <code>namespaces</code> used in types), now you can directly use exported types (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/654\">#654</a>) (<a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/590100688f66b9a7591f1f46a02de0cc6967032c\">5901006</a>)</li>\n</ul>\n</blockquote>\n</details>\n<details>\n<summary>Changelog</summary>\n<p><em>Sourced from <a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/blob/master/CHANGELOG.md\">copy-webpack-plugin's changelog</a>.</em></p>\n<blockquote>\n<h2><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/compare/v10.1.0...v10.2.0\">10.2.0</a> (2021-12-16)</h2>\n<h3>Features</h3>\n<ul>\n<li>removed cjs wrapper and generated types in commonjs format (<code>export =</code> and <code>namespaces</code> used in types), now you can directly use exported types (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/654\">#654</a>) (<a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/590100688f66b9a7591f1f46a02de0cc6967032c\">5901006</a>)</li>\n</ul>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/3b65dc34a83c69d0fa5b07ef06b3c46666467f02\"><code>3b65dc3</code></a> chore(release): 10.2.0</li>\n<li><a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/commit/590100688f66b9a7591f1f46a02de0cc6967032c\"><code>5901006</code></a> feat: drop cjs wrapper (<a href=\"https://github-redirect.dependabot.com/webpack-contrib/copy-webpack-plugin/issues/654\">#654</a>)</li>\n<li>See full diff in <a href=\"https://github.com/webpack-contrib/copy-webpack-plugin/compare/v10.1.0...v10.2.0\">compare view</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=copy-webpack-plugin&package-manager=npm_and_yarn&previous-version=10.1.0&new-version=10.2.0)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits that have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after your CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge and block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually\n- `@dependabot ignore this major version` will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)\n\n\n</details>",
          "timestamp": "2021-12-17T12:53:49Z",
          "tree_id": "3f5c759d42e46095a9bac1f88642b4d06cdf3a04",
          "url": "https://github.com/raskad/boa/commit/fed019d94fc84e115283ec4522f26c5ed9c5527b"
        },
        "date": 1639779055344,
        "tool": "criterion",
        "benches": [
          {
            "name": "Create Realm",
            "value": 363.27,
            "range": "+/- 6.480",
            "unit": "ns"
          },
          {
            "name": "Symbols (Execution)",
            "value": 4.5635,
            "range": "+/- 0.098",
            "unit": "us"
          },
          {
            "name": "For loop (Execution)",
            "value": 19.665,
            "range": "+/- 0.328",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 2.3467,
            "range": "+/- 0.033",
            "unit": "ms"
          },
          {
            "name": "",
            "value": 7.2658,
            "range": "+/- 0.123",
            "unit": "us"
          },
          {
            "name": "",
            "value": 2.7478,
            "range": "+/- 0.037",
            "unit": "ms"
          },
          {
            "name": "Array pop (Execution)",
            "value": 876.76,
            "range": "+/- 11.310",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.3876,
            "range": "+/- 0.061",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.7299,
            "range": "+/- 0.114",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.2393,
            "range": "+/- 0.108",
            "unit": "us"
          },
          {
            "name": "",
            "value": 10.236,
            "range": "+/- 0.231",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution)",
            "value": 10.055,
            "range": "+/- 0.162",
            "unit": "us"
          },
          {
            "name": "",
            "value": 13.025,
            "range": "+/- 0.201",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution) #2",
            "value": 13.112,
            "range": "+/- 0.217",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.0057,
            "range": "+/- 0.093",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.3706,
            "range": "+/- 0.168",
            "unit": "us"
          },
          {
            "name": "String copy (Execution)",
            "value": 5.0168,
            "range": "+/- 0.075",
            "unit": "us"
          },
          {
            "name": "",
            "value": 3.3865,
            "range": "+/- 0.057",
            "unit": "us"
          },
          {
            "name": "",
            "value": 4.3041,
            "range": "+/- 0.072",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.6967,
            "range": "+/- 0.108",
            "unit": "us"
          },
          {
            "name": "",
            "value": 248.51,
            "range": "+/- 3.250",
            "unit": "ns"
          },
          {
            "name": "Clean js (Execution)",
            "value": 691.05,
            "range": "+/- 7.990",
            "unit": "us"
          },
          {
            "name": "Mini js (Execution)",
            "value": 636.53,
            "range": "+/- 10.400",
            "unit": "us"
          },
          {
            "name": "Symbols (Full)",
            "value": 360.96,
            "range": "+/- 6.210",
            "unit": "us"
          },
          {
            "name": "For loop (Full)",
            "value": 404.33,
            "range": "+/- 6.550",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Full)",
            "value": 2.7448,
            "range": "+/- 0.036",
            "unit": "ms"
          },
          {
            "name": "Array access (Full)",
            "value": 392.16,
            "range": "+/- 5.350",
            "unit": "us"
          },
          {
            "name": "Array creation (Full)",
            "value": 3.0743,
            "range": "+/- 0.045",
            "unit": "ms"
          },
          {
            "name": "Array pop (Full)",
            "value": 1.3739,
            "range": "+/- 0.019",
            "unit": "ms"
          },
          {
            "name": "Object Creation (Full)",
            "value": 387.12,
            "range": "+/- 4.920",
            "unit": "us"
          },
          {
            "name": "",
            "value": 383.24,
            "range": "+/- 5.250",
            "unit": "us"
          },
          {
            "name": "",
            "value": 384.79,
            "range": "+/- 6.860",
            "unit": "us"
          },
          {
            "name": "",
            "value": 381.87,
            "range": "+/- 6.590",
            "unit": "us"
          },
          {
            "name": "RegExp (Full)",
            "value": 381.3,
            "range": "+/- 5.420",
            "unit": "us"
          },
          {
            "name": "RegExp Literal (Full)",
            "value": 403.2,
            "range": "+/- 4.650",
            "unit": "us"
          },
          {
            "name": "RegExp (Full) #2",
            "value": 403.83,
            "range": "+/- 5.870",
            "unit": "us"
          },
          {
            "name": "",
            "value": 383.28,
            "range": "+/- 7.160",
            "unit": "us"
          },
          {
            "name": "",
            "value": 387.34,
            "range": "+/- 5.580",
            "unit": "us"
          },
          {
            "name": "String copy (Full)",
            "value": 385.14,
            "range": "+/- 9.520",
            "unit": "us"
          },
          {
            "name": "",
            "value": 376.23,
            "range": "+/- 5.520",
            "unit": "us"
          },
          {
            "name": "",
            "value": 386.37,
            "range": "+/- 5.250",
            "unit": "us"
          },
          {
            "name": "",
            "value": 378.95,
            "range": "+/- 5.610",
            "unit": "us"
          },
          {
            "name": "",
            "value": 361.04,
            "range": "+/- 5.040",
            "unit": "us"
          },
          {
            "name": "Clean js (Full)",
            "value": 1.1035,
            "range": "+/- 0.018",
            "unit": "ms"
          },
          {
            "name": "Mini js (Full)",
            "value": 1.0697,
            "range": "+/- 0.016",
            "unit": "ms"
          },
          {
            "name": "Expression (Parser)",
            "value": 5.5297,
            "range": "+/- 0.110",
            "unit": "us"
          },
          {
            "name": "Hello World (Parser)",
            "value": 3.2498,
            "range": "+/- 0.054",
            "unit": "us"
          },
          {
            "name": "For loop (Parser)",
            "value": 15.072,
            "range": "+/- 0.192",
            "unit": "us"
          },
          {
            "name": "Long file (Parser)",
            "value": 715.13,
            "range": "+/- 8.710",
            "unit": "ns"
          },
          {
            "name": "Goal Symbols (Parser)",
            "value": 11.436,
            "range": "+/- 0.202",
            "unit": "us"
          },
          {
            "name": "Clean js (Parser)",
            "value": 32.13,
            "range": "+/- 0.563",
            "unit": "us"
          },
          {
            "name": "Mini js (Parser)",
            "value": 29.378,
            "range": "+/- 0.328",
            "unit": "us"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "razican@protonmail.ch",
            "name": "Iban Eguia",
            "username": "Razican"
          },
          "committer": {
            "email": "razican@protonmail.ch",
            "name": "Iban Eguia",
            "username": "Razican"
          },
          "distinct": true,
          "id": "039c46ba7b3d6d672bfe7c6bc395677e1240874b",
          "message": "Removed a bunch of warnings and clippy errors (#1754)\n\nThis Pull Request fixes some warnings and clips errors. It conflicts with the VM/non-VM PR, so should probably go in first, so that this branch gets properly updated and we get the list of real warnings/errors there.",
          "timestamp": "2021-12-23T17:43:15Z",
          "tree_id": "a58638a3e680d9d3775df1ee7a317f4eeeb68ed7",
          "url": "https://github.com/raskad/boa/commit/039c46ba7b3d6d672bfe7c6bc395677e1240874b"
        },
        "date": 1640311635951,
        "tool": "criterion",
        "benches": [
          {
            "name": "Create Realm",
            "value": 345.9,
            "range": "+/- 0.050",
            "unit": "ns"
          },
          {
            "name": "Symbols (Execution)",
            "value": 4.1965,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "For loop (Execution)",
            "value": 18.281,
            "range": "+/- 0.008",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 2.2443,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "",
            "value": 6.7643,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "",
            "value": 2.799,
            "range": "+/- 0.002",
            "unit": "ms"
          },
          {
            "name": "Array pop (Execution)",
            "value": 873.11,
            "range": "+/- 3.530",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.1786,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.411,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.9444,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "",
            "value": 9.5762,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution)",
            "value": 9.648,
            "range": "+/- 0.008",
            "unit": "us"
          },
          {
            "name": "",
            "value": 12.767,
            "range": "+/- 0.008",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution) #2",
            "value": 12.631,
            "range": "+/- 0.111",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.733,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.9541,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "String copy (Execution)",
            "value": 4.713,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "",
            "value": 3.2516,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "",
            "value": 4.1574,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.2707,
            "range": "+/- 0.014",
            "unit": "us"
          },
          {
            "name": "",
            "value": 232.83,
            "range": "+/- 0.060",
            "unit": "ns"
          },
          {
            "name": "Clean js (Execution)",
            "value": 672.45,
            "range": "+/- 0.910",
            "unit": "us"
          },
          {
            "name": "Mini js (Execution)",
            "value": 620.18,
            "range": "+/- 0.710",
            "unit": "us"
          },
          {
            "name": "Symbols (Full)",
            "value": 341.41,
            "range": "+/- 0.270",
            "unit": "us"
          },
          {
            "name": "For loop (Full)",
            "value": 375.73,
            "range": "+/- 0.190",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Full)",
            "value": 2.6379,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "Array access (Full)",
            "value": 370.31,
            "range": "+/- 0.260",
            "unit": "us"
          },
          {
            "name": "Array creation (Full)",
            "value": 2.9614,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "Array pop (Full)",
            "value": 1.3341,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "Object Creation (Full)",
            "value": 356.11,
            "range": "+/- 0.590",
            "unit": "us"
          },
          {
            "name": "",
            "value": 361.4,
            "range": "+/- 0.220",
            "unit": "us"
          },
          {
            "name": "",
            "value": 360.48,
            "range": "+/- 0.330",
            "unit": "us"
          },
          {
            "name": "",
            "value": 364.47,
            "range": "+/- 0.450",
            "unit": "us"
          },
          {
            "name": "RegExp (Full)",
            "value": 363.03,
            "range": "+/- 0.130",
            "unit": "us"
          },
          {
            "name": "RegExp Literal (Full)",
            "value": 370.7,
            "range": "+/- 0.200",
            "unit": "us"
          },
          {
            "name": "RegExp (Full) #2",
            "value": 376.04,
            "range": "+/- 0.160",
            "unit": "us"
          },
          {
            "name": "",
            "value": 355.98,
            "range": "+/- 0.170",
            "unit": "us"
          },
          {
            "name": "",
            "value": 363.84,
            "range": "+/- 0.320",
            "unit": "us"
          },
          {
            "name": "String copy (Full)",
            "value": 351.36,
            "range": "+/- 0.100",
            "unit": "us"
          },
          {
            "name": "",
            "value": 351.82,
            "range": "+/- 0.330",
            "unit": "us"
          },
          {
            "name": "",
            "value": 356.03,
            "range": "+/- 0.330",
            "unit": "us"
          },
          {
            "name": "",
            "value": 358.79,
            "range": "+/- 0.330",
            "unit": "us"
          },
          {
            "name": "",
            "value": 340.43,
            "range": "+/- 0.280",
            "unit": "us"
          },
          {
            "name": "Clean js (Full)",
            "value": 1.0542,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "Mini js (Full)",
            "value": 997.39,
            "range": "+/- 1.050",
            "unit": "us"
          },
          {
            "name": "Expression (Parser)",
            "value": 5.1582,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "Hello World (Parser)",
            "value": 3.1032,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "For loop (Parser)",
            "value": 15.057,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "Long file (Parser)",
            "value": 722.18,
            "range": "+/- 0.190",
            "unit": "ns"
          },
          {
            "name": "Goal Symbols (Parser)",
            "value": 11.008,
            "range": "+/- 0.006",
            "unit": "us"
          },
          {
            "name": "Clean js (Parser)",
            "value": 31.803,
            "range": "+/- 0.010",
            "unit": "us"
          },
          {
            "name": "Mini js (Parser)",
            "value": 27.753,
            "range": "+/- 0.023",
            "unit": "us"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "936006+jasonwilliams@users.noreply.github.com",
            "name": "Jason Williams",
            "username": "jasonwilliams"
          },
          "committer": {
            "email": "noreply@github.com",
            "name": "GitHub",
            "username": "web-flow"
          },
          "distinct": true,
          "id": "dfb3df5bf2c920262a0250d4b924201e78373541",
          "message": "Start removing non-VM path (#1747)",
          "timestamp": "2021-12-25T18:56:36+01:00",
          "tree_id": "699f9b045c443fc5d27154b330f12abe1a5ef6c6",
          "url": "https://github.com/raskad/boa/commit/dfb3df5bf2c920262a0250d4b924201e78373541"
        },
        "date": 1640471139644,
        "tool": "criterion",
        "benches": [
          {
            "name": "Create Realm",
            "value": 336.17,
            "range": "+/- 0.040",
            "unit": "ns"
          },
          {
            "name": "Symbols (Parser)",
            "value": 4.5952,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "For loop (Parser)",
            "value": 15.086,
            "range": "+/- 0.022",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Parser)",
            "value": 17.594,
            "range": "+/- 0.009",
            "unit": "us"
          },
          {
            "name": "",
            "value": 10.111,
            "range": "+/- 0.008",
            "unit": "us"
          },
          {
            "name": "",
            "value": 10.851,
            "range": "+/- 0.009",
            "unit": "us"
          },
          {
            "name": "",
            "value": 11.687,
            "range": "+/- 0.007",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.8552,
            "range": "+/- 0.006",
            "unit": "us"
          },
          {
            "name": "",
            "value": 9.1629,
            "range": "+/- 0.007",
            "unit": "us"
          },
          {
            "name": "RegExp Literal (Parser)",
            "value": 8.7713,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "RegExp (Parser)",
            "value": 10.985,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "Array access (Parser)",
            "value": 13.031,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "Array creation (Parser)",
            "value": 14.622,
            "range": "+/- 0.010",
            "unit": "us"
          },
          {
            "name": "Array pop (Parser)",
            "value": 151.64,
            "range": "+/- 0.030",
            "unit": "us"
          },
          {
            "name": "",
            "value": 8.1623,
            "range": "+/- 0.009",
            "unit": "us"
          },
          {
            "name": "",
            "value": 11.917,
            "range": "+/- 0.008",
            "unit": "us"
          },
          {
            "name": "String copy (Parser)",
            "value": 6.1275,
            "range": "+/- 0.011",
            "unit": "us"
          },
          {
            "name": "",
            "value": 12.063,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "",
            "value": 15.273,
            "range": "+/- 0.008",
            "unit": "us"
          },
          {
            "name": "",
            "value": 15.043,
            "range": "+/- 0.006",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.0827,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "Clean js (Parser)",
            "value": 31.744,
            "range": "+/- 0.011",
            "unit": "us"
          },
          {
            "name": "Mini js (Parser)",
            "value": 27.681,
            "range": "+/- 0.011",
            "unit": "us"
          },
          {
            "name": "Symbols (Compiler)",
            "value": 804.37,
            "range": "+/- 0.610",
            "unit": "ns"
          },
          {
            "name": "For loop (Compiler)",
            "value": 2.4072,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Compiler)",
            "value": 2.8321,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "",
            "value": 1.4803,
            "range": "+/- 0.000",
            "unit": "us"
          },
          {
            "name": "",
            "value": 1.5759,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "",
            "value": 1.8804,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 1.4865,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "",
            "value": 1.4971,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "",
            "value": 1.8092,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "RegExp (Compiler)",
            "value": 1.8089,
            "range": "+/- 0.000",
            "unit": "us"
          },
          {
            "name": "Array access (Compiler)",
            "value": 1.4389,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "",
            "value": 2.2245,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "Array pop (Compiler)",
            "value": 7.2661,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 1.7883,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "",
            "value": 2.5333,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "String copy (Compiler)",
            "value": 1.2557,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "",
            "value": 1.6617,
            "range": "+/- 0.000",
            "unit": "us"
          },
          {
            "name": "",
            "value": 2.0089,
            "range": "+/- 0.000",
            "unit": "us"
          },
          {
            "name": "",
            "value": 2.4565,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "",
            "value": 965.11,
            "range": "+/- 0.400",
            "unit": "ns"
          },
          {
            "name": "Clean js (Compiler)",
            "value": 5.4965,
            "range": "+/- 0.015",
            "unit": "us"
          },
          {
            "name": "Mini js (Compiler)",
            "value": 5.2596,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "Symbols (Execution)",
            "value": 5.3017,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "For loop (Execution)",
            "value": 46.314,
            "range": "+/- 0.021",
            "unit": "us"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 2.8671,
            "range": "+/- 0.002",
            "unit": "ms"
          },
          {
            "name": "",
            "value": 6.4886,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.6709,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.1172,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "",
            "value": 10.17,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "",
            "value": 10.133,
            "range": "+/- 0.010",
            "unit": "us"
          },
          {
            "name": "",
            "value": 13.122,
            "range": "+/- 0.006",
            "unit": "us"
          },
          {
            "name": "RegExp (Execution)",
            "value": 13.121,
            "range": "+/- 0.009",
            "unit": "us"
          },
          {
            "name": "",
            "value": 10.768,
            "range": "+/- 0.004",
            "unit": "us"
          },
          {
            "name": "",
            "value": 3.1928,
            "range": "+/- 0.002",
            "unit": "ms"
          },
          {
            "name": "Array pop (Execution)",
            "value": 1.3489,
            "range": "+/- 0.001",
            "unit": "ms"
          },
          {
            "name": "",
            "value": 6.4419,
            "range": "+/- 0.005",
            "unit": "us"
          },
          {
            "name": "",
            "value": 7.7,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "String copy (Execution)",
            "value": 5.6572,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 5.512,
            "range": "+/- 0.002",
            "unit": "us"
          },
          {
            "name": "",
            "value": 6.9111,
            "range": "+/- 0.015",
            "unit": "us"
          },
          {
            "name": "",
            "value": 8.8531,
            "range": "+/- 0.003",
            "unit": "us"
          },
          {
            "name": "",
            "value": 2.2343,
            "range": "+/- 0.001",
            "unit": "us"
          },
          {
            "name": "Clean js (Execution)",
            "value": 1.4495,
            "range": "+/- 0.002",
            "unit": "ms"
          },
          {
            "name": "Mini js (Execution)",
            "value": 1.3368,
            "range": "+/- 0.002",
            "unit": "ms"
          }
        ]
      },
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
          "id": "89d91f5b10bae4c85c923d1a7683b8cbd48f701f",
          "message": "Bump benchmark-action/github-action-benchmark from 1.11.2 to 1.11.3 (#1769)\n\nBumps [benchmark-action/github-action-benchmark](https://github.com/benchmark-action/github-action-benchmark) from 1.11.2 to 1.11.3.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a href=\"https://github.com/benchmark-action/github-action-benchmark/releases\">benchmark-action/github-action-benchmark's releases</a>.</em></p>\n<blockquote>\n<h2>v1.11.3</h2>\n<p>Fix: fix trailing whitespace characters in cargo benchmarks (<a href=\"https://github-redirect.dependabot.com/benchmark-action/github-action-benchmark/issues/97\">#97</a>)</p>\n</blockquote>\n</details>\n<details>\n<summary>Changelog</summary>\n<p><em>Sourced from <a href=\"https://github.com/benchmark-action/github-action-benchmark/blob/master/CHANGELOG.md\">benchmark-action/github-action-benchmark's changelog</a>.</em></p>\n<blockquote>\n<h1><a href=\"https://github.com/benchmark-action/github-action-benchmark/releases/tag/v1.11.3\">v1.11.3</a> - 31 Dec 2021</h1>\n<ul>\n<li><strong>Fix:</strong> Fix trailing whitespace characters in cargo benchmarks (<a href=\"https://github-redirect.dependabot.com/benchmark-action/github-action-benchmark/issues/97\">#97</a>)</li>\n</ul>\n<p><!-- raw HTML omitted --><!-- raw HTML omitted --></p>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a href=\"https://github.com/benchmark-action/github-action-benchmark/commit/1c1a8fb0ca538ff5572ed02039d91a610726c19e\"><code>1c1a8fb</code></a> v1.11.3</li>\n<li>See full diff in <a href=\"https://github.com/benchmark-action/github-action-benchmark/compare/v1.11.2...v1.11.3\">compare view</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=benchmark-action/github-action-benchmark&package-manager=github_actions&previous-version=1.11.2&new-version=1.11.3)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits that have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after your CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge and block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually\n- `@dependabot ignore this major version` will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)\n\n\n</details>",
          "timestamp": "2022-01-03T10:43:36Z",
          "tree_id": "1a62602b4a1462a602e8e0b3173db58b99ce0e61",
          "url": "https://github.com/raskad/boa/commit/89d91f5b10bae4c85c923d1a7683b8cbd48f701f"
        },
        "date": 1641862969554,
        "tool": "cargo",
        "benches": [
          {
            "name": "Create Realm",
            "value": 340,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Parser)",
            "value": 4763,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Parser)",
            "value": 15580,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Parser)",
            "value": 18331,
            "range": "± 26",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Parser)",
            "value": 10612,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Parser)",
            "value": 11300,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Parser)",
            "value": 12235,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Parser)",
            "value": 7198,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Parser)",
            "value": 9591,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Parser)",
            "value": 9258,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Parser)",
            "value": 11532,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Parser)",
            "value": 13381,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Parser)",
            "value": 15027,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Parser)",
            "value": 160040,
            "range": "± 220",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Parser)",
            "value": 8568,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Parser)",
            "value": 12483,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Parser)",
            "value": 6362,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Parser)",
            "value": 12554,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Parser)",
            "value": 16013,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Parser)",
            "value": 15863,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Parser)",
            "value": 6210,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Parser)",
            "value": 32759,
            "range": "± 175",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Parser)",
            "value": 28492,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Compiler)",
            "value": 796,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Compiler)",
            "value": 2406,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Compiler)",
            "value": 2809,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Compiler)",
            "value": 1473,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Compiler)",
            "value": 1567,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Compiler)",
            "value": 1886,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Compiler)",
            "value": 1492,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Compiler)",
            "value": 1493,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Compiler)",
            "value": 1844,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Compiler)",
            "value": 1836,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Compiler)",
            "value": 1443,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Compiler)",
            "value": 2233,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Compiler)",
            "value": 7285,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Compiler)",
            "value": 1793,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Compiler)",
            "value": 2518,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Compiler)",
            "value": 1256,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Compiler)",
            "value": 1688,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Compiler)",
            "value": 2035,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Compiler)",
            "value": 2477,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Compiler)",
            "value": 965,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Compiler)",
            "value": 5493,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Compiler)",
            "value": 5387,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Execution)",
            "value": 5236,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Execution)",
            "value": 45909,
            "range": "± 130",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 2868670,
            "range": "± 3766",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Execution)",
            "value": 6452,
            "range": "± 28",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Execution)",
            "value": 6637,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Execution)",
            "value": 7012,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Execution)",
            "value": 10082,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Execution)",
            "value": 10146,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Execution)",
            "value": 13084,
            "range": "± 48",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Execution)",
            "value": 13224,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Execution)",
            "value": 10742,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Execution)",
            "value": 3185222,
            "range": "± 3819",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Execution)",
            "value": 1337537,
            "range": "± 4451",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Execution)",
            "value": 6497,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Execution)",
            "value": 7781,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Execution)",
            "value": 5652,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Execution)",
            "value": 5434,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Execution)",
            "value": 6882,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Execution)",
            "value": 8696,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Execution)",
            "value": 2191,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Execution)",
            "value": 1449159,
            "range": "± 10042",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Execution)",
            "value": 1339832,
            "range": "± 8561",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "7fba7c0c45c6a79114d01721f41ca27722fdbd5c",
          "message": "Bump indexmap from 1.7.0 to 1.8.0 (#1776)\n\nBumps [indexmap](https://github.com/bluss/indexmap) from 1.7.0 to 1.8.0.\n<details>\n<summary>Changelog</summary>\n<p><em>Sourced from <a href=\"https://github.com/bluss/indexmap/blob/master/RELEASES.rst\">indexmap's changelog</a>.</em></p>\n<blockquote>\n<ul>\n<li>\n<p>1.8.0</p>\n<ul>\n<li>\n<p>The new <code>IndexMap::into_keys</code> and <code>IndexMap::into_values</code> will consume\nthe map into keys or values, respectively, matching Rust 1.54's <code>HashMap</code>\nmethods, by <a href=\"https://github.com/taiki-e\"><code>@​taiki-e</code></a> in PR 195_.</p>\n</li>\n<li>\n<p>More of the iterator types implement <code>Debug</code>, <code>ExactSizeIterator</code>, and\n<code>FusedIterator</code>, by <a href=\"https://github.com/cuviper\"><code>@​cuviper</code></a> in PR 196_.</p>\n</li>\n<li>\n<p><code>IndexMap</code> and <code>IndexSet</code> now implement rayon's <code>ParallelDrainRange</code>,\nby <a href=\"https://github.com/cuviper\"><code>@​cuviper</code></a> in PR 197_.</p>\n</li>\n<li>\n<p><code>IndexMap::with_hasher</code> and <code>IndexSet::with_hasher</code> are now <code>const</code>\nfunctions, allowing static maps and sets, by <a href=\"https://github.com/mwillsey\"><code>@​mwillsey</code></a> in PR 203_.</p>\n</li>\n<li>\n<p><code>IndexMap</code> and <code>IndexSet</code> now implement <code>From</code> for arrays, matching\nRust 1.56's implementation for <code>HashMap</code>, by <a href=\"https://github.com/rouge8\"><code>@​rouge8</code></a> in PR 205_.</p>\n</li>\n<li>\n<p><code>IndexMap</code> and <code>IndexSet</code> now have methods <code>sort_unstable_keys</code>,\n<code>sort_unstable_by</code>, <code>sorted_unstable_by</code>, and <code>par_*</code> equivalents,\nwhich sort in-place without preserving the order of equal items, by\n<a href=\"https://github.com/bhgomes\"><code>@​bhgomes</code></a> in PR 211_.</p>\n</li>\n</ul>\n</li>\n</ul>\n<p>.. _195: <a href=\"https://github-redirect.dependabot.com/bluss/indexmap/pull/195\">bluss/indexmap#195</a>\n.. _196: <a href=\"https://github-redirect.dependabot.com/bluss/indexmap/pull/196\">bluss/indexmap#196</a>\n.. _197: <a href=\"https://github-redirect.dependabot.com/bluss/indexmap/pull/197\">bluss/indexmap#197</a>\n.. _203: <a href=\"https://github-redirect.dependabot.com/bluss/indexmap/pull/203\">bluss/indexmap#203</a>\n.. _205: <a href=\"https://github-redirect.dependabot.com/bluss/indexmap/pull/205\">bluss/indexmap#205</a>\n.. _211: <a href=\"https://github-redirect.dependabot.com/bluss/indexmap/pull/211\">bluss/indexmap#211</a></p>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a href=\"https://github.com/bluss/indexmap/commit/916d1c96d2070d736c0ab5d5ba294b1c5593f009\"><code>916d1c9</code></a> Merge pull request <a href=\"https://github-redirect.dependabot.com/bluss/indexmap/issues/213\">#213</a> from cuviper/release-1.7.1</li>\n<li><a href=\"https://github.com/bluss/indexmap/commit/5386d2bf703f48550f9ac6e03c4e28b09cbc689e\"><code>5386d2b</code></a> Release 1.8.0 instead</li>\n<li><a href=\"https://github.com/bluss/indexmap/commit/f090281240c05639c665170a2c633c96adfacc07\"><code>f090281</code></a> Release 1.7.1</li>\n<li><a href=\"https://github.com/bluss/indexmap/commit/5a14f7bb8af6e3c8c4fe52bdd2978da07126cbbe\"><code>5a14f7b</code></a> Move recent changes to RELEASES.rst</li>\n<li><a href=\"https://github.com/bluss/indexmap/commit/13468f20f51666969b588f0bff7b1749726bf8ca\"><code>13468f2</code></a> Merge pull request <a href=\"https://github-redirect.dependabot.com/bluss/indexmap/issues/211\">#211</a> from bhgomes/add-sort-unstable-methods</li>\n<li><a href=\"https://github.com/bluss/indexmap/commit/8bb46ca2e4cc192ab86b6dc80015d8b5a424fe4b\"><code>8bb46ca</code></a> Merge pull request <a href=\"https://github-redirect.dependabot.com/bluss/indexmap/issues/205\">#205</a> from rouge8/from-array</li>\n<li><a href=\"https://github.com/bluss/indexmap/commit/6fca269adf18b1dd0ef0e62f5e8744c7cba51725\"><code>6fca269</code></a> No extra space is used in unstable sorts</li>\n<li><a href=\"https://github.com/bluss/indexmap/commit/5d2ce528b3c431722581526b175a51528ae0efa0\"><code>5d2ce52</code></a> Require rustc 1.51+ for <code>IndexMap::from(array)</code> and <code>IndexSet::from(array)</code></li>\n<li><a href=\"https://github.com/bluss/indexmap/commit/f0159f656d95d19b681e63b827538f6d0ca3367b\"><code>f0159f6</code></a> Add <code>IndexMap::from(array)</code> and <code>IndexSet::from(array)</code></li>\n<li><a href=\"https://github.com/bluss/indexmap/commit/4d6dde35b59009e6097a58c6ebbb0cb9b549709d\"><code>4d6dde3</code></a> Merge pull request <a href=\"https://github-redirect.dependabot.com/bluss/indexmap/issues/197\">#197</a> from cuviper/par_drain</li>\n<li>Additional commits viewable in <a href=\"https://github.com/bluss/indexmap/compare/1.7.0...1.8.0\">compare view</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=indexmap&package-manager=cargo&previous-version=1.7.0&new-version=1.8.0)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits that have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after your CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge and block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually\n- `@dependabot ignore this major version` will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)\n\n\n</details>",
          "timestamp": "2022-01-11T17:41:09Z",
          "tree_id": "dc87106f2a219587c8f3058d1a7bfbd584e9a42c",
          "url": "https://github.com/raskad/boa/commit/7fba7c0c45c6a79114d01721f41ca27722fdbd5c"
        },
        "date": 1641939101860,
        "tool": "cargo",
        "benches": [
          {
            "name": "Create Realm",
            "value": 342,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Parser)",
            "value": 4601,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Parser)",
            "value": 14983,
            "range": "± 188",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Parser)",
            "value": 17621,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Parser)",
            "value": 10195,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Parser)",
            "value": 10898,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Parser)",
            "value": 11722,
            "range": "± 17",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Parser)",
            "value": 7050,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Parser)",
            "value": 9242,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Parser)",
            "value": 8918,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Parser)",
            "value": 11122,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Parser)",
            "value": 13034,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Parser)",
            "value": 14598,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Parser)",
            "value": 149990,
            "range": "± 191",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Parser)",
            "value": 8144,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Parser)",
            "value": 11892,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Parser)",
            "value": 6039,
            "range": "± 15",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Parser)",
            "value": 12158,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Parser)",
            "value": 15445,
            "range": "± 50",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Parser)",
            "value": 15159,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Parser)",
            "value": 6123,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Parser)",
            "value": 31699,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Parser)",
            "value": 27655,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Compiler)",
            "value": 781,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Compiler)",
            "value": 2377,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Compiler)",
            "value": 2813,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Compiler)",
            "value": 1455,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Compiler)",
            "value": 1550,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Compiler)",
            "value": 1886,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Compiler)",
            "value": 1473,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Compiler)",
            "value": 1475,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Compiler)",
            "value": 1796,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Compiler)",
            "value": 1797,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Compiler)",
            "value": 1412,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Compiler)",
            "value": 2198,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Compiler)",
            "value": 7062,
            "range": "± 240",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Compiler)",
            "value": 1775,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Compiler)",
            "value": 2503,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Compiler)",
            "value": 1239,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Compiler)",
            "value": 1648,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Compiler)",
            "value": 2007,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Compiler)",
            "value": 2446,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Compiler)",
            "value": 1012,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Compiler)",
            "value": 5530,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Compiler)",
            "value": 5336,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Execution)",
            "value": 5225,
            "range": "± 11",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Execution)",
            "value": 46367,
            "range": "± 152",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 2923030,
            "range": "± 3589",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Execution)",
            "value": 6333,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Execution)",
            "value": 6584,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Execution)",
            "value": 6896,
            "range": "± 22",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Execution)",
            "value": 10122,
            "range": "± 195",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Execution)",
            "value": 10144,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Execution)",
            "value": 13260,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Execution)",
            "value": 13339,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Execution)",
            "value": 10908,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Execution)",
            "value": 3437731,
            "range": "± 8673",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Execution)",
            "value": 1396048,
            "range": "± 8840",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Execution)",
            "value": 6354,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Execution)",
            "value": 7628,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Execution)",
            "value": 5533,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Execution)",
            "value": 5274,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Execution)",
            "value": 6709,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Execution)",
            "value": 8633,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Execution)",
            "value": 2186,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Execution)",
            "value": 1467463,
            "range": "± 8252",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Execution)",
            "value": 1358321,
            "range": "± 6099",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "jase.williams@gmail.com",
            "name": "Jason Williams",
            "username": "jasonwilliams"
          },
          "committer": {
            "email": "jase.williams@gmail.com",
            "name": "Jason Williams",
            "username": "jasonwilliams"
          },
          "distinct": true,
          "id": "2300d87e227242ce12c4880ae14ce50e6b698386",
          "message": "add more timers on object functions (#1775)\n\n```\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Item                                           | Self time | % of total time | Time     | Item count |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| run                                            | 14.27ms   | 15.545          | 161.26ms | 56         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::__get_own_property__                   | 9.28ms    | 10.115          | 12.67ms  | 5412       |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| LexicalEnvironment::get_binding_value          | 9.10ms    | 9.918           | 22.00ms  | 1066       |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::validate_and_apply_property_descriptor | 6.12ms    | 6.669           | 6.12ms   | 677        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::ordinary_set                           | 4.07ms    | 4.434           | 39.14ms  | 818        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::ordinary_get_own_property              | 3.60ms    | 3.922           | 3.60ms   | 5720       |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::__call__                               | 3.22ms    | 3.505           | 103.95ms | 410        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::ordinary_define_own_property           | 3.10ms    | 3.379           | 10.90ms  | 677        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::ordinary_has_property                  | 2.95ms    | 3.209           | 7.17ms   | 1772       |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::__has_property__                       | 2.85ms    | 3.107           | 10.02ms  | 1772       |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::ordinary_get                           | 2.85ms    | 3.104           | 8.14ms   | 1632       |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::__get__                                | 2.81ms    | 3.063           | 10.95ms  | 1632       |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - GetName                                 | 2.56ms    | 2.789           | 24.56ms  | 1066       |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::__define_own_property__                | 2.48ms    | 2.704           | 13.58ms  | 521        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - SetName                                 | 1.81ms    | 1.972           | 9.52ms   | 202        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - Call                                    | 1.35ms    | 1.473           | 103.28ms | 356        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::__set__                                | 1.29ms    | 1.401           | 40.43ms  | 818        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - GetPropertyByName                       | 1.24ms    | 1.354           | 4.95ms   | 355        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Date                                           | 1.07ms    | 1.171           | 1.08ms   | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::__get_prototype_of__                   | 1.06ms    | 1.151           | 1.21ms   | 621        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - GetPropertyByValue                      | 1.05ms    | 1.143           | 1.91ms   | 154        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| create_intrinsics                              | 871.52µs  | 0.949           | 7.92ms   | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Array                                          | 761.02µs  | 0.829           | 763.84µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Opcode retrieval                               | 756.35µs  | 0.824           | 756.35µs | 4941       |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Math                                           | 750.16µs  | 0.817           | 753.72µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object                                         | 618.59µs  | 0.674           | 620.77µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| String                                         | 611.83µs  | 0.667           | 614.46µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| RegExp                                         | 419.63µs  | 0.457           | 421.09µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| next()                                         | 339.03µs  | 0.369           | 849.73µs | 96         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| console                                        | 332.59µs  | 0.362           | 334.02µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Identifier                                     | 318.53µs  | 0.347           | 337.39µs | 30         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - DefInitArg                              | 305.43µs  | 0.333           | 305.43µs | 54         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Reflect                                        | 265.70µs  | 0.289           | 267.45µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - Dup                                     | 260.49µs  | 0.284           | 260.49µs | 555        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Number                                         | 225.72µs  | 0.246           | 305.09µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| BigInt64Array                                  | 220.31µs  | 0.240           | 220.54µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Map                                            | 208.40µs  | 0.227           | 209.97µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - PushInt8                                | 201.64µs  | 0.220           | 201.64µs | 402        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Set                                            | 196.16µs  | 0.214           | 197.64µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - Pop                                     | 191.73µs  | 0.209           | 191.73µs | 455        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Symbol                                         | 186.95µs  | 0.204           | 188.13µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::__is_extensible__                      | 159.01µs  | 0.173           | 159.01µs | 677        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - LessThan                                | 152.95µs  | 0.167           | 152.95µs | 202        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::ordinary_get_prototype_of              | 152.01µs  | 0.166           | 152.01µs | 621        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Main                                           | 150.58µs  | 0.164           | 82.11ms  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - Inc                                     | 134.81µs  | 0.147           | 134.81µs | 200        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| AssignmentExpression                           | 113.77µs  | 0.124           | 4.08ms   | 21         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - JumpIfFalse                             | 103.93µs  | 0.113           | 103.93µs | 202        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| function                                       | 103.75µs  | 0.113           | 104.33µs | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| MemberExpression                               | 97.28µs   | 0.106           | 2.30ms   | 26         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| make_builtin_fn: next                          | 94.48µs   | 0.103           | 100.99µs | 6          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| From<JsObject>                                 | 83.55µs   | 0.091           | 83.55µs  | 2582       |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - Jump                                    | 83.45µs   | 0.091           | 83.45µs  | 202        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| ArrayBuffer                                    | 82.91µs   | 0.090           | 84.38µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Realm::create                                  | 82.24µs   | 0.090           | 93.74µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| BigInt                                         | 79.60µs   | 0.087           | 81.04µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| LeftHandSIdeExpression                         | 76.08µs   | 0.083           | 3.04ms   | 25         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| NumberLiteral                                  | 70.64µs   | 0.077           | 99.63µs  | 7          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| MultiplicativeExpression                       | 68.45µs   | 0.075           | 3.38ms   | 24         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - Mul                                     | 66.50µs   | 0.072           | 66.50µs  | 100        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| StatementList                                  | 66.01µs   | 0.072           | 3.70ms   | 3          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - GreaterThan                             | 61.54µs   | 0.067           | 61.54µs  | 100        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - GetFunction                             | 52.91µs   | 0.058           | 175.00µs | 2          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Float32Array                                   | 52.62µs   | 0.057           | 52.82µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| From<String>                                   | 50.52µs   | 0.055           | 50.52µs  | 431        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| ExponentiationExpression                       | 48.15µs   | 0.052           | 3.31ms   | 25         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| PrimaryExpression                              | 47.01µs   | 0.051           | 1.82ms   | 25         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - LogicalAnd                              | 46.79µs   | 0.051           | 46.79µs  | 100        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Boolean                                        | 42.78µs   | 0.047           | 43.08µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Int8Array                                      | 41.96µs   | 0.046           | 42.18µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Float64Array                                   | 41.36µs   | 0.045           | 41.57µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| BigUint64Array                                 | 41.12µs   | 0.045           | 41.34µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Uint16Array                                    | 40.82µs   | 0.044           | 41.03µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Uint8Array                                     | 40.68µs   | 0.044           | 40.89µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Uint32Array                                    | 40.61µs   | 0.044           | 40.81µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| JSON                                           | 40.59µs   | 0.044           | 41.73µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Int32Array                                     | 40.57µs   | 0.044           | 40.79µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Uint8ClampedArray                              | 40.47µs   | 0.044           | 40.69µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Int16Array                                     | 40.37µs   | 0.044           | 40.57µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| BitwiseANDExpression                           | 39.90µs   | 0.043           | 3.56ms   | 21         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Error                                          | 39.58µs   | 0.043           | 40.62µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| UpdateExpression                               | 39.50µs   | 0.043           | 3.08ms   | 25         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - RestParameterPop                        | 39.29µs   | 0.043           | 39.29µs  | 55         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Arguments                                      | 37.25µs   | 0.041           | 689.94µs | 7          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Relation Expression                            | 36.74µs   | 0.040           | 3.49ms   | 21         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| AdditiveExpression                             | 36.23µs   | 0.039           | 3.42ms   | 24         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| SyntaxError                                    | 34.38µs   | 0.037           | 35.85µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| ShiftExpression                                | 34.17µs   | 0.037           | 3.45ms   | 24         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| ReferenceError                                 | 33.10µs   | 0.036           | 34.17µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| TypeError                                      | 32.79µs   | 0.036           | 33.79µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| EvalError                                      | 32.17µs   | 0.035           | 33.43µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| URIError                                       | 32.05µs   | 0.035           | 33.35µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| ShortCircuitExpression                         | 32.02µs   | 0.035           | 3.65ms   | 20         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| BitwiseORExpression                            | 30.85µs   | 0.034           | 3.62ms   | 21         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| EqualityExpression                             | 30.82µs   | 0.034           | 3.52ms   | 21         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| BitwiseXORExpression                           | 30.80µs   | 0.034           | 3.59ms   | 21         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| ConditionalExpression                          | 29.84µs   | 0.033           | 3.68ms   | 20         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Operator                                       | 28.55µs   | 0.031           | 30.56µs  | 13         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| ForStatement                                   | 28.09µs   | 0.031           | 1.18ms   | 2          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Proxy                                          | 28.02µs   | 0.031           | 28.19µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| cursor::next_char()                            | 27.88µs   | 0.030           | 27.88µs  | 147        |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| cursor::next_is_ascii_pred()                   | 26.11µs   | 0.028           | 27.33µs  | 21         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| VariableStatement                              | 25.01µs   | 0.027           | 224.24µs | 2          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Intl                                           | 23.92µs   | 0.026           | 24.90µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Statement                                      | 23.60µs   | 0.026           | 4.26ms   | 9          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| RangeError                                     | 23.47µs   | 0.026           | 24.56µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - DefInitVar                              | 20.60µs   | 0.022           | 49.41µs  | 2          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - Return                                  | 20.47µs   | 0.022           | 20.47µs  | 55         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Expression                                     | 20.13µs   | 0.022           | 3.27ms   | 13         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| CallExpression                                 | 19.32µs   | 0.021           | 714.52µs | 6          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| make_builtin_fn: parseInt                      | 19.31µs   | 0.021           | 20.46µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Iterator Prototype                             | 18.91µs   | 0.021           | 18.99µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| String Iterator                                | 18.66µs   | 0.020           | 37.92µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| make_builtin_fn: isNaN                         | 18.43µs   | 0.020           | 19.50µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| make_builtin_fn: parseFloat                    | 18.37µs   | 0.020           | 19.41µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| make_builtin_fn: isFinite                      | 18.11µs   | 0.020           | 19.13µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| ArrowFunction                                  | 16.54µs   | 0.018           | 80.56µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| cursor::next_is()                              | 16.24µs   | 0.018           | 16.40µs  | 5          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - PushZero                                | 15.39µs   | 0.017           | 15.39µs  | 56         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - PushUndefined                           | 13.76µs   | 0.015           | 13.76µs  | 55         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| UnaryExpression                                | 11.52µs   | 0.013           | 3.75ms   | 2          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| LexicalEnvironment::new                        | 11.15µs   | 0.012           | 11.20µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| cursor::peek_char()                            | 10.27µs   | 0.011           | 10.27µs  | 78         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| LexicalEnvironment::has_binding                | 10.22µs   | 0.011           | 28.81µs  | 2          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| ArrayIterator                                  | 10.09µs   | 0.011           | 28.84µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| StatementListItem                              | 9.83µs    | 0.011           | 3.57ms   | 7          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::__construct__                          | 9.80µs    | 0.011           | 31.22µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - PushDeclarativeEnvironment              | 9.31µs    | 0.010           | 9.71µs   | 2          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - PushNewArray                            | 9.19µs    | 0.010           | 24.97µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| ExpressionStatement                            | 9.06µs    | 0.010           | 2.80ms   | 5          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| ForInIterator                                  | 8.71µs    | 0.009           | 26.32µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| SetIterator                                    | 8.67µs    | 0.009           | 26.13µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| RegExp String Iterator                         | 8.47µs    | 0.009           | 25.15µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| MapIterator                                    | 8.47µs    | 0.009           | 25.54µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| FunctionExpression                             | 7.08µs    | 0.008           | 1.77ms   | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| SpreadLiteral                                  | 6.89µs    | 0.008           | 23.29µs  | 5          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| new_declarative_environment                    | 6.76µs    | 0.007           | 6.76µs   | 59         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| FunctionStatementList                          | 5.80µs    | 0.006           | 1.82ms   | 2          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - New                                     | 5.65µs    | 0.006           | 36.90µs  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| cursor::peek()                                 | 4.65µs    | 0.005           | 4.65µs   | 59         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| FormalParameters                               | 4.54µs    | 0.005           | 6.33µs   | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - This                                    | 3.50µs    | 0.004           | 3.53µs   | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Object::get_prototype_from_constructor         | 3.42µs    | 0.004           | 9.07µs   | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| cursor::set_goal()                             | 3.33µs    | 0.004           | 3.33µs   | 99         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Initializer                                    | 3.29µs    | 0.004           | 161.59µs | 2          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| BindingIdentifier                              | 3.27µs    | 0.004           | 3.27µs   | 3          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| ArrayLiteral                                   | 2.42µs    | 0.003           | 3.98µs   | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| globalThis                                     | 1.35µs    | 0.001           | 1.38µs   | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - PopEnvironment                          | 1.23µs    | 0.001           | 1.23µs   | 2          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| cursor::next_byte()                            | 943.00ns  | 0.001           | 943.00ns | 11         |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - Swap                                    | 841.00ns  | 0.001           | 841.00ns | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - LogicalNot                              | 661.00ns  | 0.001           | 661.00ns | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - PopOnReturnAdd                          | 351.00ns  | 0.000           | 351.00ns | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| INST - PopOnReturnSub                          | 211.00ns  | 0.000           | 211.00ns | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Infinity                                       | 160.00ns  | 0.000           | 160.00ns | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| undefined                                      | 140.00ns  | 0.000           | 140.00ns | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| NaN                                            | 130.00ns  | 0.000           | 130.00ns | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Execute                                        | 70.00ns   | 0.000           | 70.00ns  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\n| Compilation                                    | 40.00ns   | 0.000           | 40.00ns  | 1          |\r\n+------------------------------------------------+-----------+-----------------+----------+------------+\r\nTotal cpu time: 91.797457ms\r\n+------+---------------+\r\n| Item | Artifact Size |\r\n+------+---------------+\r\n```",
          "timestamp": "2022-01-11T21:43:47Z",
          "tree_id": "fd056e45d3fd22bfe6f0d7a60ac8ae083cc64090",
          "url": "https://github.com/raskad/boa/commit/2300d87e227242ce12c4880ae14ce50e6b698386"
        },
        "date": 1641939388152,
        "tool": "cargo",
        "benches": [
          {
            "name": "Create Realm",
            "value": 392,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Parser)",
            "value": 4939,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Parser)",
            "value": 16696,
            "range": "± 329",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Parser)",
            "value": 19370,
            "range": "± 783",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Parser)",
            "value": 11135,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Parser)",
            "value": 11991,
            "range": "± 143",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Parser)",
            "value": 12719,
            "range": "± 187",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Parser)",
            "value": 7546,
            "range": "± 134",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Parser)",
            "value": 9969,
            "range": "± 195",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Parser)",
            "value": 9690,
            "range": "± 524",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Parser)",
            "value": 12033,
            "range": "± 210",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Parser)",
            "value": 13785,
            "range": "± 371",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Parser)",
            "value": 15676,
            "range": "± 405",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Parser)",
            "value": 171182,
            "range": "± 2680",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Parser)",
            "value": 8854,
            "range": "± 118",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Parser)",
            "value": 13041,
            "range": "± 532",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Parser)",
            "value": 6382,
            "range": "± 182",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Parser)",
            "value": 12967,
            "range": "± 340",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Parser)",
            "value": 17111,
            "range": "± 269",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Parser)",
            "value": 16868,
            "range": "± 295",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Parser)",
            "value": 6609,
            "range": "± 125",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Parser)",
            "value": 34039,
            "range": "± 727",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Parser)",
            "value": 29828,
            "range": "± 824",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Compiler)",
            "value": 920,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Compiler)",
            "value": 2927,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Compiler)",
            "value": 3239,
            "range": "± 112",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Compiler)",
            "value": 1691,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Compiler)",
            "value": 1844,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Compiler)",
            "value": 2258,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Compiler)",
            "value": 1729,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Compiler)",
            "value": 1703,
            "range": "± 39",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Compiler)",
            "value": 2099,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Compiler)",
            "value": 2137,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Compiler)",
            "value": 1711,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Compiler)",
            "value": 2663,
            "range": "± 57",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Compiler)",
            "value": 7856,
            "range": "± 318",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Compiler)",
            "value": 2026,
            "range": "± 65",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Compiler)",
            "value": 3007,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Compiler)",
            "value": 1489,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Compiler)",
            "value": 1894,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Compiler)",
            "value": 2232,
            "range": "± 46",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Compiler)",
            "value": 2764,
            "range": "± 78",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Compiler)",
            "value": 1183,
            "range": "± 19",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Compiler)",
            "value": 6490,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Compiler)",
            "value": 6284,
            "range": "± 150",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Execution)",
            "value": 6107,
            "range": "± 324",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Execution)",
            "value": 53646,
            "range": "± 1388",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 3363787,
            "range": "± 56723",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Execution)",
            "value": 7492,
            "range": "± 274",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Execution)",
            "value": 7439,
            "range": "± 194",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Execution)",
            "value": 7960,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Execution)",
            "value": 11827,
            "range": "± 245",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Execution)",
            "value": 11938,
            "range": "± 203",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Execution)",
            "value": 15026,
            "range": "± 317",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Execution)",
            "value": 15249,
            "range": "± 324",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Execution)",
            "value": 12250,
            "range": "± 257",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Execution)",
            "value": 3954740,
            "range": "± 80976",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Execution)",
            "value": 1604152,
            "range": "± 29816",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Execution)",
            "value": 7325,
            "range": "± 190",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Execution)",
            "value": 8788,
            "range": "± 238",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Execution)",
            "value": 6374,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Execution)",
            "value": 6220,
            "range": "± 101",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Execution)",
            "value": 7780,
            "range": "± 139",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Execution)",
            "value": 9847,
            "range": "± 221",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Execution)",
            "value": 2539,
            "range": "± 75",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Execution)",
            "value": 1672298,
            "range": "± 36252",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Execution)",
            "value": 1575820,
            "range": "± 33791",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "5c4d970119df1ef3a951ae2803fab2d15758ac92",
          "message": "Bump serde_json from 1.0.73 to 1.0.75 (#1785)\n\nBumps [serde_json](https://github.com/serde-rs/json) from 1.0.73 to 1.0.75.\n<details>\n<summary>Release notes</summary>\n<p><em>Sourced from <a href=\"https://github.com/serde-rs/json/releases\">serde_json's releases</a>.</em></p>\n<blockquote>\n<h2>v1.0.74</h2>\n<ul>\n<li>Allow creating RawValues from references to unsized values (<a href=\"https://github-redirect.dependabot.com/serde-rs/json/issues/841\">#841</a>, thanks <a href=\"https://github.com/EFanZh\"><code>@​EFanZh</code></a>)</li>\n</ul>\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a href=\"https://github.com/serde-rs/json/commit/a22b686f499e12146cf9a8932d2aebce7c9f2fc3\"><code>a22b686</code></a> Release 1.0.75</li>\n<li><a href=\"https://github.com/serde-rs/json/commit/36c43bfed5aa695f8d7a8138de585d425b042ed3\"><code>36c43bf</code></a> Merge pull request <a href=\"https://github-redirect.dependabot.com/serde-rs/json/issues/848\">#848</a> from serde-rs/num</li>\n<li><a href=\"https://github.com/serde-rs/json/commit/d54138145514758ea9475baf77cd5bf4788e9eaf\"><code>d541381</code></a> Deserialize small numbers as integers in arbitrary_precision</li>\n<li><a href=\"https://github.com/serde-rs/json/commit/0ca5a69d734c4f5a0185e93de491b45e25ca39b6\"><code>0ca5a69</code></a> Add regression test for issue 845</li>\n<li><a href=\"https://github.com/serde-rs/json/commit/66919777d0c31addd190c7a48ec78145a270294d\"><code>6691977</code></a> Disable buggy iter_not_returning_iterator lint</li>\n<li><a href=\"https://github.com/serde-rs/json/commit/aebe84cb09663d0c3371273fb20d838c2e752bcd\"><code>aebe84c</code></a> Raise toolchain version for preserve_order to rust 1.46</li>\n<li><a href=\"https://github.com/serde-rs/json/commit/3f459308f5055e9a4b1b611a77dad07132011e8d\"><code>3f45930</code></a> Set miriflags once for whole miri job</li>\n<li><a href=\"https://github.com/serde-rs/json/commit/c79d9ad2e1da0ea3d8e21841404df78ba6f82435\"><code>c79d9ad</code></a> Run miri also with some features enabled</li>\n<li><a href=\"https://github.com/serde-rs/json/commit/58d40de6ed22bf0c80bbe2443dd00f265281aa2d\"><code>58d40de</code></a> Release 1.0.74</li>\n<li><a href=\"https://github.com/serde-rs/json/commit/ef7794f87f43d22fc74d1e38aa2cec1d6bcec12e\"><code>ef7794f</code></a> Detect warnings in CI</li>\n<li>Additional commits viewable in <a href=\"https://github.com/serde-rs/json/compare/v1.0.73...v1.0.75\">compare view</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=serde_json&package-manager=cargo&previous-version=1.0.73&new-version=1.0.75)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits that have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after your CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge and block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually\n- `@dependabot ignore this major version` will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)\n\n\n</details>",
          "timestamp": "2022-01-17T23:20:06Z",
          "tree_id": "88712b77b2ee9b6c127e1d00caae016f13738b51",
          "url": "https://github.com/raskad/boa/commit/5c4d970119df1ef3a951ae2803fab2d15758ac92"
        },
        "date": 1642532266050,
        "tool": "cargo",
        "benches": [
          {
            "name": "Create Realm",
            "value": 403,
            "range": "± 6",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Parser)",
            "value": 4821,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Parser)",
            "value": 16210,
            "range": "± 428",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Parser)",
            "value": 19040,
            "range": "± 396",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Parser)",
            "value": 10704,
            "range": "± 300",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Parser)",
            "value": 11334,
            "range": "± 295",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Parser)",
            "value": 12040,
            "range": "± 289",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Parser)",
            "value": 7376,
            "range": "± 130",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Parser)",
            "value": 9769,
            "range": "± 192",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Parser)",
            "value": 9283,
            "range": "± 225",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Parser)",
            "value": 11740,
            "range": "± 263",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Parser)",
            "value": 13917,
            "range": "± 240",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Parser)",
            "value": 15664,
            "range": "± 309",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Parser)",
            "value": 168108,
            "range": "± 3879",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Parser)",
            "value": 8585,
            "range": "± 203",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Parser)",
            "value": 12887,
            "range": "± 213",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Parser)",
            "value": 6406,
            "range": "± 116",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Parser)",
            "value": 13049,
            "range": "± 228",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Parser)",
            "value": 16538,
            "range": "± 247",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Parser)",
            "value": 16292,
            "range": "± 311",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Parser)",
            "value": 6429,
            "range": "± 109",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Parser)",
            "value": 33657,
            "range": "± 585",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Parser)",
            "value": 29605,
            "range": "± 503",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Compiler)",
            "value": 884,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Compiler)",
            "value": 2751,
            "range": "± 61",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Compiler)",
            "value": 3217,
            "range": "± 89",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Compiler)",
            "value": 1721,
            "range": "± 37",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Compiler)",
            "value": 1857,
            "range": "± 36",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Compiler)",
            "value": 2156,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Compiler)",
            "value": 1711,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Compiler)",
            "value": 1715,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Compiler)",
            "value": 2073,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Compiler)",
            "value": 2123,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Compiler)",
            "value": 1669,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Compiler)",
            "value": 2564,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Compiler)",
            "value": 8189,
            "range": "± 146",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Compiler)",
            "value": 2069,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Compiler)",
            "value": 2943,
            "range": "± 47",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Compiler)",
            "value": 1467,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Compiler)",
            "value": 1900,
            "range": "± 33",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Compiler)",
            "value": 2291,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Compiler)",
            "value": 2834,
            "range": "± 92",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Compiler)",
            "value": 1133,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Compiler)",
            "value": 6600,
            "range": "± 94",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Compiler)",
            "value": 6268,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Execution)",
            "value": 6047,
            "range": "± 124",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Execution)",
            "value": 54360,
            "range": "± 1265",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 3369073,
            "range": "± 70149",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Execution)",
            "value": 7508,
            "range": "± 136",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Execution)",
            "value": 7762,
            "range": "± 138",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Execution)",
            "value": 8313,
            "range": "± 350",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Execution)",
            "value": 11409,
            "range": "± 317",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Execution)",
            "value": 11520,
            "range": "± 336",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Execution)",
            "value": 14875,
            "range": "± 217",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Execution)",
            "value": 14684,
            "range": "± 403",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Execution)",
            "value": 12380,
            "range": "± 239",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Execution)",
            "value": 3622804,
            "range": "± 122298",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Execution)",
            "value": 1520275,
            "range": "± 39564",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Execution)",
            "value": 7412,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Execution)",
            "value": 8959,
            "range": "± 305",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Execution)",
            "value": 6333,
            "range": "± 237",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Execution)",
            "value": 6389,
            "range": "± 175",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Execution)",
            "value": 8087,
            "range": "± 159",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Execution)",
            "value": 9861,
            "range": "± 191",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Execution)",
            "value": 2569,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Execution)",
            "value": 1687965,
            "range": "± 34097",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Execution)",
            "value": 1553546,
            "range": "± 33266",
            "unit": "ns/iter"
          }
        ]
      },
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
          "id": "76a27ce2a5ac953c18fcc9f9376fe9e44af61a0e",
          "message": "Bump wasm-bindgen from 0.2.78 to 0.2.79 (#1789)\n\nBumps [wasm-bindgen](https://github.com/rustwasm/wasm-bindgen) from 0.2.78 to 0.2.79.\n<details>\n<summary>Changelog</summary>\n<p><em>Sourced from <a href=\"https://github.com/rustwasm/wasm-bindgen/blob/main/CHANGELOG.md\">wasm-bindgen's changelog</a>.</em></p>\n<blockquote>\n<h2>0.2.79</h2>\n<p>Released 2022-01-19.</p>\n<p><a href=\"https://github.com/rustwasm/wasm-bindgen/compare/0.2.78...0.2.79\">changes</a></p>\n<hr />\n</blockquote>\n</details>\n<details>\n<summary>Commits</summary>\n<ul>\n<li><a href=\"https://github.com/rustwasm/wasm-bindgen/commit/9b0d40c7a9330c7d36de2eb0c1fb4cf462cab86d\"><code>9b0d40c</code></a> Version bump (<a href=\"https://github-redirect.dependabot.com/rustwasm/wasm-bindgen/issues/2772\">#2772</a>)</li>\n<li><a href=\"https://github.com/rustwasm/wasm-bindgen/commit/3e507e649c8bbd10a3eae7bf160b2fb78ce90307\"><code>3e507e6</code></a> Fix missing wasm_bindgen attribute (<a href=\"https://github-redirect.dependabot.com/rustwasm/wasm-bindgen/issues/2773\">#2773</a>)</li>\n<li><a href=\"https://github.com/rustwasm/wasm-bindgen/commit/3701c9d6be800f504a76551aabe234db25577680\"><code>3701c9d</code></a> Implement OptionIntoWasmAbi for Closure references (<a href=\"https://github-redirect.dependabot.com/rustwasm/wasm-bindgen/issues/2768\">#2768</a>)</li>\n<li><a href=\"https://github.com/rustwasm/wasm-bindgen/commit/c25c1f4b092ac9fc20a5482962b874ad68b0ee3c\"><code>c25c1f4</code></a> Typo (<a href=\"https://github-redirect.dependabot.com/rustwasm/wasm-bindgen/issues/2765\">#2765</a>)</li>\n<li><a href=\"https://github.com/rustwasm/wasm-bindgen/commit/39423ed01acef65e2b35b80a5fdcb19f26ec2361\"><code>39423ed</code></a> Fix some more tests</li>\n<li><a href=\"https://github.com/rustwasm/wasm-bindgen/commit/d68ceed95608636ef8b0512b4a7a05c1397874c4\"><code>d68ceed</code></a> spellcheck (<a href=\"https://github-redirect.dependabot.com/rustwasm/wasm-bindgen/issues/2762\">#2762</a>)</li>\n<li><a href=\"https://github.com/rustwasm/wasm-bindgen/commit/f158a75e9e4cb55783be97eca3877f60ecdbaedb\"><code>f158a75</code></a> Update test expectations</li>\n<li><a href=\"https://github.com/rustwasm/wasm-bindgen/commit/8aa58ac01952ab95cb6b1834ae5a659111c43540\"><code>8aa58ac</code></a> Fix macro hygiene in wasm_bindgen_test (<a href=\"https://github-redirect.dependabot.com/rustwasm/wasm-bindgen/issues/2748\">#2748</a>)</li>\n<li><a href=\"https://github.com/rustwasm/wasm-bindgen/commit/c515cbf5683333e64e6b6aad684683009c3bd6c2\"><code>c515cbf</code></a> Documentation link updated. (<a href=\"https://github-redirect.dependabot.com/rustwasm/wasm-bindgen/issues/2749\">#2749</a>)</li>\n<li><a href=\"https://github.com/rustwasm/wasm-bindgen/commit/ac87c8215bdd28d6aa0e12705996238a78227f8c\"><code>ac87c82</code></a> Fix <code>fn () -&gt; Result\\&lt;T, JsValue&gt;</code> leaking stack space (<a href=\"https://github-redirect.dependabot.com/rustwasm/wasm-bindgen/issues/2710\">#2710</a>)</li>\n<li>Additional commits viewable in <a href=\"https://github.com/rustwasm/wasm-bindgen/compare/0.2.78...0.2.79\">compare view</a></li>\n</ul>\n</details>\n<br />\n\n\n[![Dependabot compatibility score](https://dependabot-badges.githubapp.com/badges/compatibility_score?dependency-name=wasm-bindgen&package-manager=cargo&previous-version=0.2.78&new-version=0.2.79)](https://docs.github.com/en/github/managing-security-vulnerabilities/about-dependabot-security-updates#about-compatibility-scores)\n\nDependabot will resolve any conflicts with this PR as long as you don't alter it yourself. You can also trigger a rebase manually by commenting `@dependabot rebase`.\n\n[//]: # (dependabot-automerge-start)\n[//]: # (dependabot-automerge-end)\n\n---\n\n<details>\n<summary>Dependabot commands and options</summary>\n<br />\n\nYou can trigger Dependabot actions by commenting on this PR:\n- `@dependabot rebase` will rebase this PR\n- `@dependabot recreate` will recreate this PR, overwriting any edits that have been made to it\n- `@dependabot merge` will merge this PR after your CI passes on it\n- `@dependabot squash and merge` will squash and merge this PR after your CI passes on it\n- `@dependabot cancel merge` will cancel a previously requested merge and block automerging\n- `@dependabot reopen` will reopen this PR if it is closed\n- `@dependabot close` will close this PR and stop Dependabot recreating it. You can achieve the same result by closing it manually\n- `@dependabot ignore this major version` will close this PR and stop Dependabot creating any more for this major version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this minor version` will close this PR and stop Dependabot creating any more for this minor version (unless you reopen the PR or upgrade to it yourself)\n- `@dependabot ignore this dependency` will close this PR and stop Dependabot creating any more for this dependency (unless you reopen the PR or upgrade to it yourself)\n\n\n</details>",
          "timestamp": "2022-01-20T23:45:52Z",
          "tree_id": "7f6a990ea0289458ba7b971a80a63189751fb22f",
          "url": "https://github.com/raskad/boa/commit/76a27ce2a5ac953c18fcc9f9376fe9e44af61a0e"
        },
        "date": 1642733569960,
        "tool": "cargo",
        "benches": [
          {
            "name": "Create Realm",
            "value": 336,
            "range": "± 0",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Parser)",
            "value": 4126,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Parser)",
            "value": 13444,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Parser)",
            "value": 17736,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Parser)",
            "value": 10327,
            "range": "± 25",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Parser)",
            "value": 11036,
            "range": "± 29",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Parser)",
            "value": 10476,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Parser)",
            "value": 6185,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Parser)",
            "value": 8227,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Parser)",
            "value": 7914,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Parser)",
            "value": 9810,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Parser)",
            "value": 11586,
            "range": "± 24",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Parser)",
            "value": 14677,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Parser)",
            "value": 132675,
            "range": "± 131",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Parser)",
            "value": 7384,
            "range": "± 8",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Parser)",
            "value": 10684,
            "range": "± 12",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Parser)",
            "value": 5407,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Parser)",
            "value": 12147,
            "range": "± 781",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Parser)",
            "value": 13736,
            "range": "± 14",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Parser)",
            "value": 13527,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Parser)",
            "value": 5343,
            "range": "± 5",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Parser)",
            "value": 31820,
            "range": "± 51",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Parser)",
            "value": 24460,
            "range": "± 83",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Compiler)",
            "value": 831,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Compiler)",
            "value": 2222,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Compiler)",
            "value": 2548,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Compiler)",
            "value": 1564,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Compiler)",
            "value": 1661,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Compiler)",
            "value": 1715,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Compiler)",
            "value": 1385,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Compiler)",
            "value": 1383,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Compiler)",
            "value": 1658,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Compiler)",
            "value": 1659,
            "range": "± 10",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Compiler)",
            "value": 1506,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Compiler)",
            "value": 2019,
            "range": "± 4",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Compiler)",
            "value": 6312,
            "range": "± 7",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Compiler)",
            "value": 1612,
            "range": "± 2",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Compiler)",
            "value": 2263,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Compiler)",
            "value": 1156,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Compiler)",
            "value": 1467,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Compiler)",
            "value": 1779,
            "range": "± 1",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Compiler)",
            "value": 2175,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Compiler)",
            "value": 887,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Compiler)",
            "value": 4981,
            "range": "± 68",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Compiler)",
            "value": 5460,
            "range": "± 9",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Execution)",
            "value": 4667,
            "range": "± 18",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Execution)",
            "value": 41605,
            "range": "± 105",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 2526750,
            "range": "± 3423",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Execution)",
            "value": 5707,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Execution)",
            "value": 5891,
            "range": "± 27",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Execution)",
            "value": 6236,
            "range": "± 32",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Execution)",
            "value": 9810,
            "range": "± 34",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Execution)",
            "value": 8684,
            "range": "± 41",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Execution)",
            "value": 13027,
            "range": "± 49",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Execution)",
            "value": 12994,
            "range": "± 40",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Execution)",
            "value": 9561,
            "range": "± 30",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Execution)",
            "value": 3199194,
            "range": "± 12396",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Execution)",
            "value": 1178119,
            "range": "± 4575",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Execution)",
            "value": 5736,
            "range": "± 23",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Execution)",
            "value": 6987,
            "range": "± 20",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Execution)",
            "value": 4955,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Execution)",
            "value": 4953,
            "range": "± 13",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Execution)",
            "value": 6171,
            "range": "± 21",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Execution)",
            "value": 7914,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Execution)",
            "value": 1910,
            "range": "± 3",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Execution)",
            "value": 1308957,
            "range": "± 10175",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Execution)",
            "value": 1202933,
            "range": "± 18982",
            "unit": "ns/iter"
          }
        ]
      },
      {
        "commit": {
          "author": {
            "email": "razican@protonmail.ch",
            "name": "Iban Eguia",
            "username": "Razican"
          },
          "committer": {
            "email": "razican@protonmail.ch",
            "name": "Iban Eguia",
            "username": "Razican"
          },
          "distinct": true,
          "id": "48185f393dbac48c4199ee5bda596240961c9f04",
          "message": "Lexer string interning (#1758)\n\nThis Pull Request is part of #279.\r\n\r\n It adds a string interner to Boa, which allows many types to not contain heap-allocated strings, and just contain a `NonZeroUsize` instead. This can move types to the stack (hopefully I'll be able to move `Token`, for example, maybe some `Node` types too.\r\n\r\nNote that the internet is for now only available in the lexer. Next steps (in this PR or future ones) would include also using interning in the parser, and finally in execution. The idea is that strings should be represented with a `Sym` until they are displayed.\r\n\r\nTalking about display. I have changed the `ParseError` type in order to not contain anything that could contain a `Sym` (basically tokens), which might be a bit faster, but what is important is that we don't depend on the interner when displaying errors.\r\n\r\nThe issue I have now is in order to display tokens. This requires the interner if we want to know identifiers, for example. The issue here is that Rust doesn't allow using a `fmt::Formatter` (only in nightly), which is making my head hurt. Maybe someone of you can find a better way of doing this.\r\n\r\nThen, about `cursor.expect()`, this is the only place where we don't have the expected token type as a static string, so it's failing to compile. We have the option of changing the type definition of `ParseError` to contain an owned string, but maybe we can avoid this by having a `&'static str` come from a `TokenKind` with the default values, such as \"identifier\" for an identifier. I wanted for you to think about it and maybe we can just add that and avoid allocations there.\r\n\r\nOh, and this depends on the VM-only branch, so that has to be merged before :)\r\n\r\nAnother thing to check: should the interner be in its own module?",
          "timestamp": "2022-01-22T18:03:31Z",
          "tree_id": "c4f7e377c97d74570d718aa5ebaae3244c7517a3",
          "url": "https://github.com/raskad/boa/commit/48185f393dbac48c4199ee5bda596240961c9f04"
        },
        "date": 1642952060975,
        "tool": "cargo",
        "benches": [
          {
            "name": "Create Realm",
            "value": 410,
            "range": "± 31",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Parser)",
            "value": 5231,
            "range": "± 312",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Parser)",
            "value": 17333,
            "range": "± 800",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Parser)",
            "value": 20089,
            "range": "± 1037",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Parser)",
            "value": 11379,
            "range": "± 748",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Parser)",
            "value": 12112,
            "range": "± 470",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Parser)",
            "value": 13259,
            "range": "± 579",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Parser)",
            "value": 7882,
            "range": "± 323",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Parser)",
            "value": 10527,
            "range": "± 532",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Parser)",
            "value": 9925,
            "range": "± 290",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Parser)",
            "value": 12502,
            "range": "± 746",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Parser)",
            "value": 14888,
            "range": "± 471",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Parser)",
            "value": 16546,
            "range": "± 685",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Parser)",
            "value": 180339,
            "range": "± 7974",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Parser)",
            "value": 9514,
            "range": "± 387",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Parser)",
            "value": 14175,
            "range": "± 569",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Parser)",
            "value": 7070,
            "range": "± 450",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Parser)",
            "value": 13658,
            "range": "± 679",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Parser)",
            "value": 17170,
            "range": "± 843",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Parser)",
            "value": 17690,
            "range": "± 2706",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Parser)",
            "value": 7187,
            "range": "± 351",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Parser)",
            "value": 38049,
            "range": "± 2859",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Parser)",
            "value": 33386,
            "range": "± 3915",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Compiler)",
            "value": 1089,
            "range": "± 81",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Compiler)",
            "value": 3185,
            "range": "± 231",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Compiler)",
            "value": 3548,
            "range": "± 139",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Compiler)",
            "value": 1989,
            "range": "± 82",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Compiler)",
            "value": 2092,
            "range": "± 120",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Compiler)",
            "value": 2471,
            "range": "± 112",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Compiler)",
            "value": 1966,
            "range": "± 162",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Compiler)",
            "value": 1972,
            "range": "± 90",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Compiler)",
            "value": 2346,
            "range": "± 164",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Compiler)",
            "value": 2299,
            "range": "± 125",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Compiler)",
            "value": 1822,
            "range": "± 98",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Compiler)",
            "value": 2835,
            "range": "± 163",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Compiler)",
            "value": 8688,
            "range": "± 709",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Compiler)",
            "value": 2201,
            "range": "± 107",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Compiler)",
            "value": 3132,
            "range": "± 160",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Compiler)",
            "value": 1620,
            "range": "± 201",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Compiler)",
            "value": 2063,
            "range": "± 103",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Compiler)",
            "value": 2453,
            "range": "± 268",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Compiler)",
            "value": 3011,
            "range": "± 122",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Compiler)",
            "value": 1153,
            "range": "± 42",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Compiler)",
            "value": 6675,
            "range": "± 287",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Compiler)",
            "value": 6496,
            "range": "± 371",
            "unit": "ns/iter"
          },
          {
            "name": "Symbols (Execution)",
            "value": 7340,
            "range": "± 388",
            "unit": "ns/iter"
          },
          {
            "name": "For loop (Execution)",
            "value": 58420,
            "range": "± 2849",
            "unit": "ns/iter"
          },
          {
            "name": "Fibonacci (Execution)",
            "value": 3774447,
            "range": "± 193571",
            "unit": "ns/iter"
          },
          {
            "name": "Object Creation (Execution)",
            "value": 8959,
            "range": "± 350",
            "unit": "ns/iter"
          },
          {
            "name": "Static Object Property Access (Execution)",
            "value": 9037,
            "range": "± 275",
            "unit": "ns/iter"
          },
          {
            "name": "Dynamic Object Property Access (Execution)",
            "value": 9652,
            "range": "± 404",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal Creation (Execution)",
            "value": 13413,
            "range": "± 1454",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Creation (Execution)",
            "value": 13403,
            "range": "± 424",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp Literal (Execution)",
            "value": 16336,
            "range": "± 2020",
            "unit": "ns/iter"
          },
          {
            "name": "RegExp (Execution)",
            "value": 16404,
            "range": "± 705",
            "unit": "ns/iter"
          },
          {
            "name": "Array access (Execution)",
            "value": 14781,
            "range": "± 954",
            "unit": "ns/iter"
          },
          {
            "name": "Array creation (Execution)",
            "value": 3845803,
            "range": "± 106723",
            "unit": "ns/iter"
          },
          {
            "name": "Array pop (Execution)",
            "value": 1606781,
            "range": "± 51966",
            "unit": "ns/iter"
          },
          {
            "name": "String concatenation (Execution)",
            "value": 8225,
            "range": "± 306",
            "unit": "ns/iter"
          },
          {
            "name": "String comparison (Execution)",
            "value": 9644,
            "range": "± 602",
            "unit": "ns/iter"
          },
          {
            "name": "String copy (Execution)",
            "value": 7190,
            "range": "± 625",
            "unit": "ns/iter"
          },
          {
            "name": "Number Object Access (Execution)",
            "value": 7158,
            "range": "± 285",
            "unit": "ns/iter"
          },
          {
            "name": "Boolean Object Access (Execution)",
            "value": 9070,
            "range": "± 253",
            "unit": "ns/iter"
          },
          {
            "name": "String Object Access (Execution)",
            "value": 11515,
            "range": "± 535",
            "unit": "ns/iter"
          },
          {
            "name": "Arithmetic operations (Execution)",
            "value": 2784,
            "range": "± 245",
            "unit": "ns/iter"
          },
          {
            "name": "Clean js (Execution)",
            "value": 1796765,
            "range": "± 82449",
            "unit": "ns/iter"
          },
          {
            "name": "Mini js (Execution)",
            "value": 1649840,
            "range": "± 72922",
            "unit": "ns/iter"
          }
        ]
      }
    ]
  }
}