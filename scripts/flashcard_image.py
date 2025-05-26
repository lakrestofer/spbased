#!/usr/bin/env python
import json
from datetime import datetime
from os import path
import os
from subprocess import PIPE, Popen
from typing import Optional
import subprocess
from typing import List

import typer

SPBASED_MODEL = "flashcard_image"

AGAIN = "again could not answer"
HARD = "hard could answer with difficulty"
GOOD = "good could answer"
EASY = "easy could answer easily"

CHOICES = [AGAIN, HARD, GOOD, EASY]

###############################################################################
# gum helpers
###############################################################################


def gum_command(cmd: str, args: List[str]):
    process = Popen(["gum", cmd] + args, stdout=PIPE, stderr=None, text=True)
    stdout, stderr = process.communicate()
    if process.returncode != 0:
        return None
    return stdout.strip()


def gum_choose(args: List[str], selected: Optional[str] = None):
    if selected is not None:
        args = args + ["--selected", selected]
    return gum_command("choose", args)


def gum_confirm(args: List[str]):
    return gum_command("confirm", args)


def gum_file(args: List[str]):
    return gum_command("file", args)


def gum_filter(args: List[str]):
    return gum_command("filter", args)


def gum_format(args: List[str]):
    return gum_command("format", args)


def gum_input(args: List[str]):
    return gum_command("input", args)


def gum_join(args: List[str]):
    return gum_command("join", args)


def gum_pager(args: List[str]):
    return gum_command("pager", args)


def gum_spin(args: List[str]):
    return gum_command("spin", args)


def gum_style(args: List[str]):
    return gum_command("style", args)


def gum_table(args: List[str]):
    return gum_command("table", args)


def gum_write(args: List[str]):
    return gum_command("write", args)


def gum_log(message, level: str = "info"):
    return gum_command("log", [f"--level={level}", message])


###############################################################################
# screenshot helpers
###############################################################################

BG_COLOR_QUESTION = "#2596be22"
BG_COLOR_ANSWER = "#b8bb2622"

def take_screenshot(output: str, bg_color: str = BG_COLOR_QUESTION):
    slurp_proc = subprocess.run(["slurp", "-b", bg_color], capture_output=True, text=True)
    if slurp_proc.returncode != 0:
        return False

    geometry = slurp_proc.stdout.strip()

    with open(output, "wb") as f:
        process = subprocess.run(["grim", "-g", geometry, "-"], stdout=f)

    if process.returncode != 0:
        return False
    return True


def gen_filename() -> str:
    now = datetime.now()
    now_str = now.strftime("%Y%m%dT%H%M%S")
    home_path = path.expanduser("~")
    file_name = f"{home_path}/vault/study/flashcard_images/{now_str}.png"
    return file_name


###############################################################################
# spbased helpers
###############################################################################


def spbased_command(args: List[str]):
    process = Popen(["spbasedctl"] + args, stdout=PIPE, stderr=PIPE, text=True)
    stdout, stderr = process.communicate()
    return stdout.strip()


def spbased_add(questions: List[str], answers: List[str]):
    data = {"questions": questions, "answers": answers}
    data_str = json.dumps(data)
    return spbased_command(
        ["items", "add", "--model", SPBASED_MODEL, "--data", data_str]
    )


def spbased_n_due_new():
    n_due = spbased_command(
        ["review", "query-count", "due", f"--filter=model=='{SPBASED_MODEL}'"]
    )
    n_new = spbased_command(
        ["review", "query-count", "new", f"--filter=model=='{SPBASED_MODEL}'"]
    )
    return (n_due, n_new)


def spbased_get_new():
    return spbased_command(
        ["review", "next", "new", f"--pre-filter=model=='{SPBASED_MODEL}'"]
    )


def spbased_get_due():
    return spbased_command(
        ["review", "next", "due", f"--pre-filter=model=='{SPBASED_MODEL}'"]
    )


def spbased_review_score(id: str, score: str):
    return spbased_command(["review", "score", id, score])


###############################################################################
# imv helpers
###############################################################################


def imv_open(images: List[str]):
    processes = []
    for image in images:
        processes.append(Popen(["imv", image], stdout=None, stderr=None, text=True))

    exit_codes = [p.wait() for p in processes]

    return exit_codes


###############################################################################
# cli
###############################################################################

app = typer.Typer()


@app.command()
def add():
    questions = []
    answers = []

    ## add questions
    gum_log("Question(s): (press escape to enter answers)")
    file_name = gen_filename()
    if not take_screenshot(file_name):
        return

    gum_log(f"screenshot: {file_name}")
    questions.append(file_name)

    gum_log("Answers(s): (press escape to finish)")
    file_name = gen_filename()
    if not take_screenshot(file_name, BG_COLOR_ANSWER):
        return
    gum_log(f"screenshot: {file_name}")
    answers.append(file_name)

    if not len(questions) or not len(answers):
        for question in questions:
            if (os.path.exists(question)):
                os.remove(question)
        for answer in answers:
            if (os.path.exists(answer)):
                os.remove(answer)
        return


    spbased_add(questions, answers)


@app.command()
def edit():
    gum_log("not implemented yet", level="error")
    pass


@app.command()
def review():
    (n_due, n_new) = spbased_n_due_new()
    (n_due, n_new) = (int(n_due), int(n_new))
    gum_log(f"You have {n_due} image flashcards that are due and {n_new} that are new")

    if not n_due and not n_new:
        return

    review_item = None

    if n_due:
        review_item = spbased_get_due()
    else:
        review_item = spbased_get_new()

    if review_item is None:
        gum_log("no item", level="warn")
        return

    # extract data
    review_item = json.loads(review_item)
    id = str(review_item["id"])
    questions = review_item["data"]["questions"]
    answer = review_item["data"]["answers"]

    # show questions
    gum_log(f"item id: {id}", level="info")
    gum_log("showing questions", level="info")
    questions_res = imv_open(questions)
    for code in questions_res:
        if code != 0:
            gum_log(f"error when opening image, code: {code}", "error")

    # show answers
    gum_log("showing question and answer", level="info")
    answer_res = imv_open(questions + answer)
    for code in answer_res:
        if code != 0:
            gum_log(f"error when opening image, code: {code}", "error")

    # grade
    res = gum_choose([f"{c}" for c in CHOICES], GOOD)

    if res is None:
        return

    res = res.split()[0].strip()

    res = spbased_review_score(id, res)

    print(res)


if __name__ == "__main__":
    app()
