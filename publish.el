;;; publish.el --- ellis
;;
;; Commentary:
;;
;; this script is executed in a build pipeline via `emacs -q publish.el`.

(require 'htmlize "~/shed/lab/cfg/emacs/el/htmlize/htmlize.el")
(require 'ox-publish)
(require 'ox-latex)
(require 'ox-man)
(require 'ox-html)
(require 'cl-lib)
(require 'ox-ascii)
(require 'org-id)

;;; Config
(setq org-html-validation-link nil
      org-html-head-include-default-style nil
      org-html-htmlize-output-type nil
      org-export-html-style-include-scripts nil
      org-export-html-style-include-default nil

      org-export-with-sub-superscripts '{}
      org-export-with-author nil
      org-export-with-timestamp nil
      org-export-with-email nil
      org-export-with-section-numbers nil
      org-export-with-toc 5
      org-export-with-title nil
      org-export-with-num nil)

;;;; Project List
(setq org-publish-project-alist
      '(("org-html"
         :author "ellis"
         :email "ellis@rwest.io"
	       :base-directory "/mnt/src/rwest_io/ox/org"
	       :base-extension "org"
         :with-date (current-time)
	       :publishing-directory "/mnt/src/rwest_io/ox/html"
         :body-only t
	       :recursive t
	       :publishing-function org-html-publish-to-html
	       :auto-sitemap nil)
        ("org-md"
         :author "ellis"
         :email "ellis@rwest.io"
	       :base-directory "/mnt/src/rwest_io/ox/org"
	       :base-extension "org"
         :with-date (current-time)
	       :publishing-directory "/mnt/src/rwest_io/ox/md"
	       :recursive t
	       :publishing-function org-md-publish-to-md
	       :auto-sitemap nil)
        ("org-man"
         :author "ellis"
         :email "ellis@rwest.io"
	       :base-directory "/mnt/src/rwest_io/ox/org"
	       :base-extension "org"
         :with-date (current-time)
	       :publishing-directory "/mnt/src/rwest_io/ox/man"
	       :recursive t
	       :publishing-function org-man-export-to-man
	       :auto-sitemap nil)
        ("org-pdf"
         :author "ellis"
         :email "ellis@rwest.io"
	       :base-directory "/mnt/src/rwest_io/ox/org"
	       :base-extension "org"
         :with-date (current-time)
	       :publishing-directory "/mnt/src/rwest_io/ox/pdf"
	       :recursive t
	       :publishing-function org-latex-publish-to-pdf
	       :auto-sitemap nil)
        ("org-txt"
         :author "ellis"
         :email "ellis@rwest.io"
	       :base-directory "/mnt/src/rwest_io/ox/org"
	       :base-extension "org"
         :with-date (current-time)
	       :publishing-directory "/mnt/src/rwest_io/ox/txt"
	       :recursive t
	       :publishing-function org-ascii-publish-to-ascii
	       :auto-sitemap nil)
	      ("all" :components ("org-html" "org-md" "org-pdf" "org-txt"))))

;;; link-types imported from main config.org
(setq org-link-abbrev-alist
      '(("google"    . "http://www.google.com/search?q=")
        ("gmap"      . "http://maps.google.com/maps?q=%s")
        ("omap"      . "http://nominatim.openstreetmap.org/search?q=%s&polygon=1")
        ("ads"       . "https://ui.adsabs.harvard.edu/search/q=%20author%3A\"%s\"")
        ("src" . "https://hg.rwest.io/%s")
        ("contrib" . "https://hg.rwest.io/%s")
        ("cdn" . "https://cdn.rwest.io/%s")))

(defvar yt-iframe-format
  ;; You may want to change your width and height.
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
;;; Execution
(org-publish-remove-all-timestamps)
(org-refile-cache-clear)
(org-publish "all" t) ; publish using current thread
