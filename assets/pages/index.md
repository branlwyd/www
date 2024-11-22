<h1 class="site-header">branland</h1>
<div class="contact-list"><a href="https://github.com/branlwyd" target="_blank">Github</a> | <a href="https://bsky.app/profile/bran.land" target="_blank">Bluesky</a> | <a href="mailto:bran@bran.land" target="_blank">Email</a></div>

Welcome to Branland! I'm Bran, and this is my personal website. I'm a Software
Engineer living in Portland. My interests include mathematics, computer science,
information security/privacy, and cryptography. I'm also into baking, fitness,
video games, board games/TTRPGs, musical theater & opera, tarot, art (especially
occult & surreal), coffee... and more.

## Projects

I have a lot of hobby software projects. Most of the software I build is at
least somewhat useful to me, but my aim is usually self-education (or just
entertainment). I do most of my coding in [Rust](https://rust-lang.org/) these
days. I'm also proficient in Go, Java, Python, C, and a number of other
languages.

Here are some of my favorite projects:

*  [This site](https://github.com/branlwyd/www): the server for this site is a
   custom-written Rust webserver. It serves HTTP using the
   [axum](https://crates.io/crates/axum) crate, and uses the
   [rustls-acme](https://crates.io/crates/rustls-acme) crate to automatically
   retrieve and renew TLS certificates from [Let's
   Encrypt](https://letsencrypt.org). The site's static content is stored as
   files in the source repository, with page content stored in Markdown format.
   During compilation, the Markdown files are converted into HTML via a build
   script, calling into a tool based on the
   [pulldown-cmark](https://crates.io/crates/pulldown-cmark) crate. The
   resulting HTML & all other static assets are then embedded into the resulting
   server binary via the [rust-embed](https://crates.io/crates/rust-embed)
   crate. This means that a single command is all that is needed to compile all
   of the various sources (code, content, and static assets) into a single
   binary. Updates to the site only require replacing the old binary with a new
   binary and restarting the service. Everything is served from memory: the
   filesystem is used only to cache TLS certificates. It receives an A on both
   the [Qualys SSL Server Test](https://ssllabs.com/ssltest) & the [Probely
   Security Headers Scan]( https://securityheaders.io), so all the requisite
   boxes are ticked.
*  [harpocrates](https://github.com/branlwyd/harpocrates): a single-user,
   self-hosted, web-based password manager, written in Go. The security model is
   novel (to my knowledge) among password managers: logging in requires a
   password, but each individual password entry also requires a touch from a
   [WebAuthn](https://en.wikipedia.org/wiki/WebAuthn) device to access. This
   would allow accessing some entries from a lower-trust machine (in a pinch),
   without potentially exposing all entries to the machine if it is
   untrustworthy. Two vault formats are supported: one equivalent to that used
   by [pass](https://www.passwordstore.org) allowing for interoperability, and
   one based on
   [nacl/secretbox](https://pkg.go.dev/golang.org/x/crypto/nacl/secretbox) which
   is much more performant. Please note that this project has not undergone a
   security review.
*  [rspd](https://github.com/branlwyd/rspd): an async TLS SNI proxy, written in
   Rust. This daemon parses enough of the TLS `ClientHello` message to determine
   the hostname indicated in the SNI extension, and then proxies the connection
   to a host based on that SNI hostname.
*  [rnccd](https://github.com/branlwyd/rnccd): a simple Namecheap Dynamic DNS
   client, written in Rust. This daemon wakes up once per minute, checks its
   public IP address, and updates a Namecheap Dynamic DNS entry using the
   [API](https://www.namecheap.com/support/knowledgebase/article.aspx/29/11/how-to-dynamically-update-the-hosts-ip-with-an-https-request/)
   whenever the IP changes.
*  [acnh\_flowers](https://github.com/branlwyd/acnh_flowers): a library allowing
   for exploration of flower genetics in the game *Animal Crossing: New
   Horizons*, written in Go. It includes functionality allowing the parsing and
   rendering of genotypes, as well as breeding genotypes or distributions on
   genotypes together. Perhaps most impressively, it includes functionality
   allowing for efficient searches for optimal breeding paths to a desired
   phenotype/genotype -- it can find a more efficient (but more complex) path to
   blue roses than the one generally used by the community in about a day on my
   machine.
*  [redird](https://github.com/branlwyd/redird): a configuration-based link
   shortener/redirector, written in Go. Also has the ability to generate
   indexes, which allows it to act as a link aggregator; the indexes can
   optionally display images in the case that the linked content is an image.
*  [rssdl](https://github.com/branlwyd/rssdl): a pure-Go daemon which watches a
   set of RSS feeds and downloads any new files (represented as links in the RSS
   feed) to a specified directory. When the feed is checked, and how frequently,
   can be configured on a per-feed basis. Uses [Bazel](https://bazel.build/) as
   a build system.
*  [bNotify](https://github.com/branlwyd/bNotify): a pure-Go daemon & Android
   app which allows you to push notifications to your phone from any computer.
   It uses [FCM](https://firebase.google.com/docs/cloud-messaging) to push the
   notifications. The notification data is encrypted & authenticated in transit.
   Please note that this project has not undergone a security review; at the
   very least, every server & the client share the same symmetric key, so
   compromise of one server negates the confidentiality & authenticity of
   messages from any server. For this reason (and perhaps others), this project
   should be considered a toy and is likely unsuitable when real security is
   required.
*  [bdcpu16](https://github.com/branlwyd/bdcpu16): a Java implementation of the
   [DCPU-16](https://raw.githubusercontent.com/gatesphere/demi-16/master/docs/dcpu-specs/dcpu-1-7.txt)
   virtual machine from Notch's cancelled 0x10c. Includes support for all
   hardware defined by the specification, an assembler, and a debugger.
