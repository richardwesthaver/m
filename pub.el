#!/usr/local/bin/emacs --script
(require 'ox-publish)
(require 'org-refile)
(require 'org-id)
(let ((default-directory  "~/shed/src/contrib/el/"))
  (normal-top-level-add-subdirs-to-load-path))
(require 'hyde "~/shed/src/cfg/emacs/lisp/hyde.el")
(require 'htmlize "~/shed/src/contrib/el/htmlize/htmlize.el")

(setq org-publish-project-alist
      '(("org-html"
         :author "ellis"
         :email "ellis@rwest.io"
	 :base-directory "~/shed/src/meta/o/x/org"
	 :base-extension "org"
         :with-date (current-time)
	 :publishing-directory "~/shed/src/meta/o/x/html"
	 :publishing-function org-html-publish-to-html
	 :htmlize-source t
	 :auto-sitemap nil)
        ("org-md"
         :author "ellis"
         :email "ellis@rwest.io"
	 :base-directory "~/shed/src/meta/o/x/org"
	 :base-extension "org"
         :with-date (current-time)
	 :publishing-directory "~/shed/src/meta/o/x/md"
	 :recursive t
	 :publishing-function org-md-publish-to-md
	 :auto-sitemap nil)
        ("org-man"
         :author "ellis"
         :email "ellis@rwest.io"
	 :base-directory "~/shed/src/meta/o/x/org"
	 :base-extension "org"
         :with-date (current-time)
	 :publishing-directory "~/shed/src/meta/o/x/man"
	 :recursive t
	 :publishing-function org-man-export-to-man
	 :auto-sitemap nil)
        ("org-pdf"
         :author "ellis"
         :email "ellis@rwest.io"
	 :base-directory "~/shed/src/meta/o/x/org"
	 :base-extension "org"
         :with-date (current-time)
	 :publishing-directory "~/shed/src/meta/o/x/pdf"
	 :recursive t
	 :publishing-function org-latex-publish-to-pdf
	 :auto-sitemap nil)
        ("org-txt"
         :author "ellis"
         :email "ellis@rwest.io"
	 :base-directory "~/shed/src/meta/o/x/org"
	 :base-extension "org"
         :with-date (current-time)
	 :publishing-directory "~/shed/src/meta/o/x/txt"
	 :recursive t
	 :publishing-function org-ascii-publish-to-ascii
	 :auto-sitemap nil)
	("all" :components ("org-html" "org-md" "org-pdf" "org-txt"))))

(setq org-link-abbrev-alist
      '(("google"    . "http://www.google.com/search?q=")
        ("gmap"      . "http://maps.google.com/maps?q=%s")
        ("omap"      . "http://nominatim.openstreetmap.org/search?q=%s&polygon=1")
        ("ads"       . "https://ui.adsabs.harvard.edu/search/q=%20author%3A\"%s\"")
	("rw" . "https://rwest.io/%s")
        ("src" . "https://hg.rwest.io/%s")
        ("contrib" . "https://hg.rwest.io/contrib/%s")
        ("cdn" . "https://rwest.io/a/%s")))

(defvar yt-iframe-format
  (concat "<iframe width=\"480\""
          " height=\"360\""
          " src=\"https://www.youtube.com/embed/%s\""
          " frameborder=\"0\""
          " allowfullscreen>%s</iframe>"))

(org-add-link-type
 "yt"
 (lambda (handle)
   (browse-url
    (concat "https://www.youtube.com/embed/"
            handle)))
 (lambda (path desc backend)
   (cl-case backend
     (html (format yt-iframe-format
                   path (or desc "")))
     (latex (format "\href{%s}{%s}"
                    path (or desc "video"))))))

(org-publish "all" t nil)
