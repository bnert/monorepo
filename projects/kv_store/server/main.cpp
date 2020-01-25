extern "C" {
  #include <unistd.h>
  #include <sys/types.h>
  #include <sys/socket.h>
  #include <sys/un.h>
  #include <stdlib.h>
}

#include <iostream>
#include <map>

typedef int FileDescriptor;
typedef int ConnId;

class ConnectionManager
{
  int connection_socket_{-1};
  struct sockaddr_un socket_info_;
  std::string conn_type_;
  std::string conn_addr_;
  std::map<ConnId, FileDescriptor> conn_map_;

  ConnectionManager()
  {
    // Nothing to do...
  }

  std::string _ParseString(std::string conn_string)
  {
    if (conn_string.find(':') == std::string::npos)
      return "Malformed protocol prefix. Ex: unix:/some/dir\n";

    int len = 0;
    for(std::string::iterator it = conn_string.begin(); it != conn_string.end() && *it != ':'; it++)
    {
      conn_type_.push_back(*it);
      len++;
    }

    // Get rid of ':'
    conn_string.erase(0, len + 1);
    conn_addr_ = conn_string;
    return "";
  }

public:
  ~ConnectionManager()
  {
    // Nothing to do
  }

  static ConnectionManager NewByValue()
  {
    return ConnectionManager();
  }

  static ConnectionManager* NewByHeap()
  {
    return new ConnectionManager();
  }

  // Conn str ex: unix:/some/dir/path/from/root
  const std::string ListenOn(std::string conn_string)
  {
    std::string err = _ParseString(conn_string);
    if (err != "")
      return err;

    unlink(conn_addr_.c_str());
    connection_socket_ = socket(AF_UNIX, SOCK_STREAM, 0);
    memset(&socket_info_, 0, sizeof(socket_info_));
    socket_info_.sun_family = AF_UNIX;
    strncpy(socket_info_.sun_path, conn_addr_.c_str(), sizeof(socket_info_.sun_path) - 1);
    if (bind(connection_socket_, (const struct sockaddr *)&socket_info_, sizeof(struct sockaddr_un)) == -1)
      return "Unable to bind\n";

    if (listen(connection_socket_, 20) == -1)
      return "Unable to listen\n";

    std::cout << "Open on: " << conn_string << std::endl;
    return "";
  }

  const std::string Close()
  {
    close(connection_socket_);
    unlink(conn_addr_.c_str());
    return "";
  }

  void Print()
  {
    std::cout << conn_type_ << std::endl << conn_addr_ << std::endl;
  }
};


int main()
{
  ConnectionManager cm = ConnectionManager::NewByValue();

  std::string err = cm.ListenOn("unix:/tmp/abcdefg.socket");
  if (err != "")
  {
    std::cout << err;
    return -1;
  }

  while(true)

  cm.Close();

  return 0;
}
