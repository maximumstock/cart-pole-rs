download:
	python3.11 -m rl_zoo3.load_from_hub --algo ppo --env CartPole-v1 -orga sb3 -f logs/

export:
	python3.11 export_model.py

.PHONY: download export
