dl:
	python3 -m rl_zoo3.load_from_hub --algo ppo --env CartPole-v1 -orga sb3 -f logs/

ex:
	python3 enjoy.py --algo ppo --env CartPole-v1  -f logs/


