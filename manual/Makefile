BUILDDIR := "build"

TEX := pdflatex

TEXARGS := --output-dir=$(BUILDDIR)

DOC := bedrock

$(BUILDDIR)/$(DOC).pdf: $(DOC).tex $(BUILDDIR)
	$(TEX) $(TEXARGS) $(DOC).tex
	$(TEX) $(TEXARGS) $(DOC).tex

$(BUILDDIR):
	mkdir -p $(BUILDDIR)
