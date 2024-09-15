import logging


formatter = logging.Formatter("[%(asctime)s][%(levelname)s]: %(message)s")

file_handler = logging.FileHandler("bfp.log", mode = "w", encoding = "utf-8", delay = True)
file_handler.setFormatter(formatter)

logger = logging.getLogger(__name__)
if logger.hasHandlers():
    logger.handlers.clear()
logger.addHandler(file_handler)
logger.setLevel(logging.DEBUG)
