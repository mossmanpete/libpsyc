ORG_PATH = /usr/share/emacs/site-lisp/org-mode
INIT = (setq load-path (cons \"/usr/share/emacs/site-lisp/org-mode\" load-path) \
             make-backup-files nil \
             org-export-author-info nil \
             org-export-creator-info nil \
             org-export-html-validation-link nil \
             org-babel-tangle-pad-newline nil \
             org-src-preserve-indentation t) \
       (require 'org-install)
#'
ORG = benchmark.org

it:
	for f in ${ORG}; do \
		emacs -Q --batch --eval \
			"(progn ${INIT} (find-file \"$$f\") \
			  (org-html-export-to-html) (kill-buffer))"; \
#			  (org-export-as-html-batch) (kill-buffer))";
	done

pdf:
	for f in ${ORG}; do \
		emacs -Q --batch --eval \
			"(progn ${INIT} (find-file \"$@.org\") \
			  (org-export-as-pdf org-export-headline-levels) (kill-buffer))"
