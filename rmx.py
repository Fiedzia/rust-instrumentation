#!/usr/bin/env python3

import sys
import struct
import argparse
import socket
import json
import binascii
import time

MAX_MSG_SIZE = 2**16-1


def prepare_msg(op, key):
    if not key:
        msg = op
    else:
        msg = u'{} {}'.format(op, key)
    bmsg = msg.encode('utf8')
    return struct.pack('>I', len(bmsg))+bmsg


def read_msg(s):
    #import pudb; pudb.set_trace()
    data = s.recv(4)
    msg_size = struct.unpack('>I', data)[0]
    if msg_size < 0 or msg_size > MAX_MSG_SIZE:
        raise Exception('invalid msg_size: {}'.format(msg_size))
    msg = s.recv(msg_size).decode('utf8')
    parsed_msg = json.loads(msg)
    return parsed_msg


def parse_args():
    parser = argparse.ArgumentParser(
        description='RMX - Rust instrumentation tool')
    parser.add_argument('-c', '--connect', dest='connection', action='store',
                        required=True, help='connection string')
    parser.add_argument('-t', '--timer', dest='timer', action='store',
                        type=float, required=False, default=None,
                        help='keep displaying results repeatedly')
    parser.add_argument('-v', '--verbose', dest='verbose', action='store_true',
                        default=False,  required=False,
                        help='be verbose')
    parser.add_argument('command', metavar='COMMAND',
                        help='GET_KEY|HAS_KEY|GET_SUBKEYS')
    parser.add_argument('key', metavar='KEY', nargs='?',
                        help='key name')

    args = parser.parse_args()
    return args

if __name__ == '__main__':

    args = parse_args()
    
    key = args.key
    cmd = args.command
    if args.connection.startswith('unix://'):
        s = socket.socket(socket.AF_UNIX, socket.SOCK_STREAM)
        s.connect(args.connection[7:])
    elif args.connection.startswith('tcp://'):
        addr, port = args.connection[6:].split(':')
        addr = addr or '127.0.0.1'
        s = socket.socket(socket.AF_INET, socket.SOCK_STREAM)
        s.connect((addr, int(port)))
    else:
        sys.stderr.write('Unknown protocol: {}\n'.format(args.connection))
        sys.exit(1)
    
    s.send(prepare_msg(cmd, key))
    res = read_msg(s)
    print('{} {} {}'.format(cmd, key, res))
    if args.timer is not None:
        while True:
            time.sleep(args.timer)
            s.send(prepare_msg(cmd, key))
            res = read_msg(s)
            print('{} {} {}'.format(cmd, key, res))
   
    
    #import pudb; pudb.set_trace()
    
    s.close()
