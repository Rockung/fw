== A single instance of Node.js runs in a single thread
== Launch a cluster of Node.js process for multi-core systems

             cluster          worker
====================================================
events       setup            
             fork             
             listening        listening
						 message          message
						 online           online
						 disconnect       disconnect
						 exit             exit
						                  error
----------------------------------------------------
properties   isMaster         id
             isWorker         process
						 settings         exitedAfterDisconnect
						 worker
						 workers
						 schedulingPolicy
----------------------------------------------------
methods      disconnect       isConnected
             fork             isDead
						 setupMaster      kill
						                  send
															
