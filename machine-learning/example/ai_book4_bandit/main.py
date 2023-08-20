import numpy as np
import math
import pandas as pd
import matplotlib.pyplot as plt
import slotarm as sa
import algos as alg

def play(algo, arms, num_sims, num_time):
    times = np.zeros(num_sims * num_time)
    rewards = np.zeros(num_sims * num_time)

    for sim in range(num_sims):
        algo.initialize(len(arms))

        for time in range(num_time):
            index = sim * num_time + time

            times[index] = time+1
            chosen_arm = algo.select_arm()
            reward = arms[chosen_arm].draw()
            rewards[index] = reward

            algo.update(chosen_arm, reward, time+1)

    return [times, rewards]

arms = (sa.SlotArm(0.3), sa.SlotArm(0.5), sa.SlotArm(0.9))
algos = (alg.EpsilonGreedy(0.1), alg.UCB1())

for algo in algos:
    results = play(algo, arms, 1000, 250)

    df = pd.DataFrame({'times': results[0], 'rewards': results[1]})
    mean = df['rewards'].groupby(df['times']).mean()
    plt.plot(mean, label=algo.label())

plt.xlabel('Step')
plt.ylabel('Average Reward')
plt.legend(loc='best')
plt.show()
