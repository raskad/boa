window.BENCHMARK_DATA = {
  "lastUpdate": 1639435337274,
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
      }
    ]
  }
}