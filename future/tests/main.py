# tests/main.py: The test file for jisp.
#
# Need be run at the root of the project (e.g. "python3 tests/main.py").

import subprocess

def show_block(title, text):
    bar = "-" * 60
    print(f"    +{bar} {title} {bar}")
    lines = text.splitlines()
    if type(text) is bytes:
        indented = b"    | " + b"\n    | ".join(lines)
        indented = indented.decode()
    else:
        indented = "    | " + "\n    | ".join(lines)
    print(indented)

def show_end():
    print("    +" + "-"*(120 + 8))

def print_captured_stdout_stderr(process):
    if len(process.stdout.strip()) != 0:
        show_block("STDOUT", process.stdout)
    if len(process.stderr.strip()) != 0:
        show_block("STDERR", process.stderr)

def e2e_unit(script, wanted):
    e2e_process = subprocess.run(["./target/release/jisp", "run", "-c", script], capture_output=True)
    if e2e_process.returncode != 0:
        print("[FAILED] E2E - Return code is not zero.")
        show_block("SCRIPT", script)
        show_block("WANTED", wanted)
        print_captured_stdout_stderr(e2e_process)
        show_end()
        return False
    elif e2e_process.stdout != bytes(wanted, 'utf-8'):
        print("[FAILED] E2E - result not matched.")
        show_block("SCRIPT", script)
        show_block("WANTED", wanted)
        print_captured_stdout_stderr(e2e_process)
        show_end()
        return False
    else:
        return True

def e2e():
    cases = [
        # OK-ed cases.
        ("1", "1"),
        ("2", "2"),

        # TODO cases.
        ("(+ 1 1)", "2")
    ]
    all_ok = True
    for case in cases:
        ok = e2e_unit(case[0], case[1])
        if not ok:
            all_ok = False
            break
    if all_ok:
        print("[  OK  ] E2E")

def main():
    # Unit test.
    unit_test_process = subprocess.run(["cargo", "test"], capture_output=True)
    if unit_test_process.returncode != 0:
        print("[FAILED] UNIT_TEST")
        print_captured_stdout_stderr(unit_test_process)
        show_end()
    else:
        print("[  OK  ] UNIT_TEST")
    
    # Build release.
    build_process = subprocess.run(["cargo", "build", "--release"], capture_output=True)
    if build_process.returncode != 0:
        print("[FAILED] BUILD_RELEASE")
        print_captured_stdout_stderr(build_process)
        show_end()
    else:
        print("[  OK  ] BUILD_RELEASE")

    # E2E.
    e2e()

main()