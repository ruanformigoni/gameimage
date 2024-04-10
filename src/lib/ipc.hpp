///
// @author      : Ruan E. Formigoni (ruanformigoni@gmail.com)
// @file        : ipc
///

#pragma once

#include <sys/ipc.h>
#include <sys/msg.h>
#include <cstring>

#include "../common.hpp"
#include "../std/concepts.hpp"
#include "../std/string.hpp"

#include "../lib/log.hpp"

namespace ns_ipc
{

namespace fs = std::filesystem;

struct message_buffer
{
  long message_type;
  char message_text[10240];
};

class Ipc
{
  private:
    key_t m_key;
    int m_message_queue_id;
    message_buffer m_buffer;
  public:
    Ipc(fs::path path_file);
    ~Ipc();
    template<ns_concept::AsString T>
    void send(T&& t);

}; // class

inline Ipc::Ipc(fs::path path_file)
{
  std::string identifier = ns_string::to_string(path_file);

  // Use a unique key for the message queue.
  if(m_key = ftok(identifier.c_str(), 65); m_key == -1 )
  {
    perror("Could not generate token for message queue");
    "Could not generate key for message queue with identifier '{}': {}"_throw(identifier, strerror(errno));
  } // if
  ns_log::write('i', "Generated message_queue key: ", m_key);

  // Connect to the message queue
  if (m_message_queue_id = msgget(m_key, 0666 | IPC_CREAT); m_message_queue_id == -1 )
  {
    perror("Could not create message queue");
    "msgget failed, could not create message queue for identifier '{}': {}"_throw(identifier, strerror(errno));
  } // if
  ns_log::write('i', "Message queue id: ", m_message_queue_id);

  m_buffer.message_type = 1;
} // Ipc::Ipc

inline Ipc::~Ipc()
{
  if ( msgctl(m_message_queue_id, IPC_RMID, NULL) == -1 )
  {
    ns_log::write('i', "Could not remove the message queue");
    perror("Could not remove message queue");
  } // if
} // Ipc::~Ipc

template<ns_concept::AsString T>
void Ipc::send(T&& t)
{
  std::string data = ns_string::to_string(t);
  // Copy the contents of std::string to the message_text buffer
  strncpy(m_buffer.message_text, data.c_str(), sizeof(m_buffer.message_text));
  // Ensure null termination
  m_buffer.message_text[sizeof(m_buffer.message_text) - 1] = '\0';
  // Send message
  if ( msgsnd(m_message_queue_id, &m_buffer, sizeof(m_buffer), 0) == -1 )
  {
    perror("Failure to send message");
  } // if
} // Ipc::send

} // namespace ns_ipc

/* vim: set expandtab fdm=marker ts=2 sw=2 tw=100 et :*/
