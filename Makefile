BRANCH=$(shell git branch --show-current)

start:
	@echo "Current branch: $(BRANCH)"
	git pull origin $(BRANCH)
	@echo "✅ Updated. Ready to work."

commit:
	@read -p "Commit message: " msg; \
	git add . && git commit -m "$$msg" && git push origin $(BRANCH); \
	echo "✅ Pushed to $(BRANCH)"

merge:
	@read -p "Branch to merge: " merge_branch; \
	git checkout main && git pull origin main; \
	git merge $$merge_branch && git push origin main; \
	echo "✅ Merged $$merge_branch into main."

new-branch:
	@read -p "New branch name: " new_branch; \
	git checkout -b $$new_branch && git push origin $$new_branch; \
	echo "✅ Created $$new_branch"
