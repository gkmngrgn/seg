# Static Email (Template) Generator

[![codecov](https://codecov.io/gh/gkmngrgn/meyl/branch/master/graph/badge.svg)](https://codecov.io/gh/gkmngrgn/meyl)

MEYL is a **developer-first** and **accessibility-first** email generator app. I
write this app to create, edit, test, and manage all emails using a template
engine. You can use it for generating single emails or you can create reusable
email templates for Mandrill, SendGrid, Zapier, etc.

You can also edit text-based email content for the visual-impaired humans or OCR
users, but if you forget that somehow, MEYL will auto-generate it for you.

## Simple Usage

```
# create new project directory
meyl new email-templates

# generate emails
meyl generate --source=email-templates/src/ \
              --destination=email-templates/dst/

# send a test mail to preview
meyl send --template=email-templates/dst/wedding-invitation/ \
          --email="john.doe@localhost"
```

## Immutable Variables

- subject
- body

## Email Folder Structure

All templates should have:

1. a subject.html,
2. and a body.html.

You don't need to create also a body_text.html but in case, MEYL will
auto-generate it for making your emails more accessible for all human being.

```
email-templates
|
|- dst                       Destination Template Folder
|  |- payroll                Template Name: Payroll
|  |  |- body.html           Email Content
|  |  |- body_text.html      Email Content (in Text Format)
|  |  |- subject.html        Subject
|  |
|  |- wedding-invitation     Template Name: Wedding Invitation
|  |  |- body.html           Email Content
|  |  |- body_text.html      Email Content in Text Format
|  |  |- subject.html        Subject
|
|- src                       Source Template Folder
|  |- base.html              Base Template
|  |- style.css              Styles
|  |- partials               Template Partials
|  |  |- button.html
|  |  |- title.html
|  |- payroll
|  |  |- body.html
|  |  |- subject.html
|  |- wedding-information
|  |  |- body.html
|  |  |- body_text.html
|  |  |- subject.html
|
```

## TO DO

- [ ] multilanguage support.
- [ ] digest css styles to html inside.
- [ ] support to embed markdown files to the email body as a variable.
- [ ] error outputs are not helpful. if there's an undefined variable, tell me which one is that.
      if a function or a filter has been used incorrectly, tell me what it is.
- [x] auto-generate executable files every release (linux, win, mac).
- [x] allow only BODY content in html files, we can add HTML and HEAD parts
      programmatically.
- [x] test rendered outputs.
- [x] change Email struct parameters: src_dir, dst_dir, template_name.
- [x] minify rendered outputs.
- [x] make css styles inline for html templates.
- [x] choose a template engine to render email contents.
- [x] I also need a arg parser for CLI.
- [x] create project skeleton.
