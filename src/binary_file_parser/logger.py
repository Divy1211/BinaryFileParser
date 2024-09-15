import logging

logging.basicConfig(
    filename = "bfp.log", encoding = 'utf-8',
    level = logging.DEBUG, filemode = "w",
    format = "[%(asctime)s][%(levelname)s]: %(message)s"
)
logger = logging.getLogger(__name__)
