mod errors {
  /* cribbed from https://github.com/apache/zookeeper/blob/trunk/src/c/include/zookeeper.h */

  pub enum ZOO_ERRORS {
    ZOK = 0, /*!< Everything is OK */

    /** System and server-side errors.
     * This is never thrown by the server, it shouldn't be used other than
     * to indicate a range. Specifically error codes greater than this
     * value, but lesser than {@link #ZAPIERROR}, are system errors. */
    ZSYSTEMERROR = -1,
    ZRUNTIMEINCONSISTENCY = -2, /*!< A runtime inconsistency was found */
    ZDATAINCONSISTENCY = -3, /*!< A data inconsistency was found */
    ZCONNECTIONLOSS = -4, /*!< Connection to the server has been lost */
    ZMARSHALLINGERROR = -5, /*!< Error while marshalling or unmarshalling data */
    ZUNIMPLEMENTED = -6, /*!< Operation is unimplemented */
    ZOPERATIONTIMEOUT = -7, /*!< Operation timeout */
    ZBADARGUMENTS = -8, /*!< Invalid arguments */
    ZINVALIDSTATE = -9, /*!< Invliad zhandle state */
    ZNEWCONFIGNOQUORUM = -13, /*!< No quorum of new config is connected and
                                   up-to-date with the leader of last commmitted
                                   config - try invoking reconfiguration after new
                                   servers are connected and synced */
    ZRECONFIGINPROGRESS = -14, /*!< Reconfiguration requested while another
                                    reconfiguration is currently in progress. This
                                    is currently not supported. Please retry. */

    /** API errors.
     * This is never thrown by the server, it shouldn't be used other than
     * to indicate a range. Specifically error codes greater than this
     * value are API errors (while values less than this indicate a
     * {@link #ZSYSTEMERROR}).
     */
    ZAPIERROR = -100,
    ZNONODE = -101, /*!< Node does not exist */
    ZNOAUTH = -102, /*!< Not authenticated */
    ZBADVERSION = -103, /*!< Version conflict */
    ZNOCHILDRENFOREPHEMERALS = -108, /*!< Ephemeral nodes may not have children */
    ZNODEEXISTS = -110, /*!< The node already exists */
    ZNOTEMPTY = -111, /*!< The node has children */
    ZSESSIONEXPIRED = -112, /*!< The session has been expired by the server */
    ZINVALIDCALLBACK = -113, /*!< Invalid callback specified */
    ZINVALIDACL = -114, /*!< Invalid ACL specified */
    ZAUTHFAILED = -115, /*!< Client authentication failed */
    ZCLOSING = -116, /*!< ZooKeeper is closing */
    ZNOTHING = -117, /*!< (not error) no server responses to process */
    ZSESSIONMOVED = -118, /*!<session moved to another server, so operation is ignored */
    ZEPHEMERALONLOCALSESSION = -120, /*!< Attempt to create ephemeral node on a local session */
    ZNOWATCHER = -121 /*!< The watcher couldn't be found */
  }
}
