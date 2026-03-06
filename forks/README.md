# Forked Dependencies

## Philosophy

**Forks are for integration patches only.** Official crates should be used whenever possible.

If a fork contains changes that aren't accepted upstream, that's a **red flag** indicating potential architectural issues in Weaver. The dependency model should align with standard Rust ecosystem patterns.

**Only in rare cases** should forks contain Weaver-specific changes, and these must be:

1. **Well documented** (why the change exists, why it can't be upstream)
2. **Justified** (explaining the architectural constraint)
3. **Minimal** (smallest possible diff)

---

## egui-toast

**Fork Repository**: <https://github.com/dominikj111/fork_egui-toast>  
**Upstream**: <https://github.com/ItsEthra/egui-toast>  
**Type**: Git submodule

### Why Forked

**Integration patches** for compatibility with Weaver Desktop's specific egui version and build configuration.

### Current Patches

*Document specific patches here when they exist. If this section is empty, this fork should be removed and the official crate used instead.*

### Working with the Submodule

**Initial clone** (for contributors):

```bash
git clone --recursive https://github.com/dominikj111/DesktopWeaver
# or if already cloned:
git submodule update --init --recursive
```

**Before making changes**: Always try to contribute patches upstream first. Forks should only contain temporary integration fixes or changes that are genuinely incompatible with upstream's goals.

**If you must patch the fork**:

```bash
cd forks/egui-toast
# Make minimal changes
git commit -am "fix: integration patch for [specific issue]"
git push origin main
cd ../..
git add forks/egui-toast
git commit -m "chore: update egui-toast submodule with integration patch

Patch addresses: [explain what and why]
Upstream issue: [link if applicable]
Removal plan: [when can this fork be removed?]"
```

**Syncing with upstream** (preferred workflow):

```bash
cd forks/egui-toast
git remote add upstream https://github.com/ItsEthra/egui-toast.git
git fetch upstream
git merge upstream/master
# Test if patches are still needed
git push origin main
cd ../..
git add forks/egui-toast
git commit -m "chore: sync egui-toast with upstream"
```

**Goal**: Minimize diff from upstream. Regularly evaluate if the fork can be replaced with the official crate.

## Adding Future Forks (Avoid If Possible)

**First**: Try these alternatives:

1. Use the official crate from crates.io
2. Submit patches upstream
3. Use Cargo patch directives for temporary fixes
4. Redesign Weaver to work with the upstream API

**Only if absolutely necessary:**

1. Create a fork repository on GitHub documenting **why** it exists
2. Add as submodule:

    ```bash
    git submodule add https://github.com/yourusername/your-fork.git forks/your-fork
    git commit -m "chore: add your-fork submodule (temporary integration patch)
    
    Reason: [explain the specific integration issue]
    Upstream: [link to upstream issue or PR]
    Removal plan: [timeline or conditions for removing this fork]"
    ```

3. Document in this README:
   - **Why** the fork exists
   - **What** specific patches it contains
   - **When** it can be removed
   - **Upstream** tracking issue/PR

**Remember**: Every fork is technical debt. Strive for zero forks.
