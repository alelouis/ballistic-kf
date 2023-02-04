import json
import zmq
import numpy as np
import pickle

context = zmq.Context()
socket = context.socket(zmq.REP)
socket.bind("tcp://*:5555")
steps = 3000
dim = 2
data = np.zeros((dim, steps))

for step in range(steps):
    message = socket.recv()
    decoded = json.loads(message)
    data[0, step], data[1, step] = decoded
    socket.send(b"ok")

with open('../data/data.pkl', 'wb') as f:
    pickle.dump(data, f)
